use std::{path::Path, process::Command, vec::Vec};

use crate::core::{CliError, Config, Container, LalResult};

/// Flags for docker run that vary for different use cases
///
/// `interactive` should be on by default, but machine accounts should turn this off
/// `privileged` is needed on some setups for `gdb` and other low level tools to work
///
/// NB: The derived default should only be used by tests (all false/zero)
#[derive(Default)]
pub struct DockerRunFlags {
    /// Pass --interactive (allows ctrl-c on builds/scripts/shell commands)
    pub interactive: bool,
    /// Pass --privileged (situational)
    pub privileged: bool,
}

/// Various ways to invoke `docker_run`
#[derive(Default, Clone)]
pub struct ShellModes {
    /// Just print the command used rather than do it
    pub printonly: bool,
    /// Attempt to forward the X11 socket and all it needs
    pub x11_forwarding: bool,
    /// Host networking
    pub host_networking: bool,
    /// Environment variables
    pub env_vars: Vec<String>,
}

/// Verifies that `id -u` and `id -g` are sane
///
/// Docker user namespaces are not properly supported by our setup,
/// so for builds to work with the default containers, user ids and group ids
/// should match inside and outside of the container.
fn permission_sanity_check() -> LalResult<(u32, u32)> {
    let uid_output = Command::new("id").arg("-u").output()?;
    let uid_str = String::from_utf8_lossy(&uid_output.stdout);
    let uid = uid_str.trim().parse::<u32>().unwrap(); // trust `id -u` is sane

    let gid_output = Command::new("id").arg("-g").output()?;
    let gid_str = String::from_utf8_lossy(&gid_output.stdout);
    let gid = gid_str.trim().parse::<u32>().unwrap(); // trust `id -g` is sane

    if uid == 0 || gid == 0 {
        return Err(CliError::DockerPermissionSafety(
            "Cannot run container as root user".into(),
            uid,
            gid,
        ));
    }

    Ok((uid, gid))
}

/// Gets the ID of a docker container
///
/// Uses the `docker images` command to find the image ID of the specified
/// container.
/// Will return a trimmed String containing the image ID requested, wrapped in
/// a Result::Ok, or CliError::DockerImageNotFound wrapped in a Result::Err if
/// docker images returns no output.
fn get_docker_image_id(container: &Container) -> LalResult<String> {
    trace!("Using docker images to find ID of container {}", container);
    let image_id_output = Command::new("docker")
        .arg("images")
        .arg("-q")
        .arg(container.to_string())
        .output()?;
    let image_id_str: String = String::from_utf8_lossy(&image_id_output.stdout).trim().into();
    match image_id_str.len() {
        0 => {
            trace!("Could not find ID");
            Err(CliError::DockerImageNotFound(container.to_string()))
        }
        _ => {
            trace!("Found ID {}", image_id_str);
            Ok(image_id_str)
        }
    }
}

/// Pulls a docker container
///
/// Uses `docker pull` to pull the specified container from the docker repository.
/// Returns Ok(()) if the command is successful, Err(CliError::SubprocessFailure)
/// if `docker pull` fails or is interrupted by a signal, Err(CliError::Io) if the
/// command status() call fails for a different reason.
fn pull_docker_image(container: &Container) -> LalResult<()> {
    trace!("Pulling container {}", container);
    let s = Command::new("docker")
        .arg("pull")
        .arg(container.to_string())
        .status()?;
    if !s.success() {
        trace!("Pull failed");
        return Err(CliError::SubprocessFailure(s.code().unwrap_or(1001)));
    };
    trace!("Pull succeeded");
    Ok(())
}

/// Builds a docker container
///
/// Uses `docker build` to build a docker container with the specified
/// instructions. It uses the --tag option to tag it with the given information.
/// Returns Ok(()) if the command is successful, Err(CliError::SubprocessFailure)
/// if `bash -c` fails or is interrupted by a signal, Err(CliError::Io) if the
/// command status() call fails for a different reason.
fn build_docker_image(container: &Container, instructions: Vec<String>) -> LalResult<()> {
    trace!("Building docker image for {}", container);
    let instruction_strings = instructions.join("\\n");
    trace!("Build instructions: \n{}", instruction_strings);
    // More safety
    let instruction_strings = instruction_strings.replace("'", "'\\''");
    let s = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "echo -e '{}' | docker build --tag {} -",
            instruction_strings, container
        ))
        .status()?;
    if !s.success() {
        trace!("Build failed");
        return Err(CliError::SubprocessFailure(s.code().unwrap_or(1001)));
    };
    trace!("Build succeeded");
    Ok(())
}

/// Fixes up docker container for use with given uid and gid
///
/// Returns a container derived from the one passed as an argument, with the `lal`
/// user having its uid and gid modified to match the ones passed.
/// The container is built if necessary (e.g. new base container from upstream)
fn fixup_docker_container(container: &Container, u: u32, g: u32) -> LalResult<Container> {
    info!("Using appropriate container for user {}:{}", u, g);
    // Find image id of regular docker container
    // We might have to pull it
    let image_id = get_docker_image_id(container).or_else(|_| {
        pull_docker_image(container)?;
        get_docker_image_id(container)
    })?;

    // Produce name and tag of modified container
    let modified_container = Container {
        name: format!("{}-u{}_g{}", container.name, u, g),
        tag: format!("from_{}", image_id),
    };

    info!("Using container {}", modified_container);

    // Try to find image id of modified container
    // If we fail we need to build it
    match get_docker_image_id(&modified_container) {
        Ok(id) => {
            info!("Found container {}, image id is {}", modified_container, id);
        }
        Err(_) => {
            let instructions: Vec<String> = vec![
                format!("FROM {}", container),
                "USER root".into(),
                format!("RUN groupmod -g {} lal && usermod -u {} lal", g, u),
                "USER lal".into(),
            ];
            info!("Attempting to build container {}...", modified_container);
            build_docker_image(&modified_container, instructions)?;
        }
    };
    trace!("Fixup for user {}:{} succeeded", u, g);
    Ok(modified_container)
}

/// Runs an arbitrary command in the configured docker container
///
/// This will mount the current directory as `~/volume` as well as a few conveniences,
/// and absorb the `Stdio` supplied by this `Command`.
pub fn docker_run(
    cfg: &Config,
    container: &Container,
    command: Vec<String>,
    flags: &DockerRunFlags,
    modes: &ShellModes,
    component_dir: &Path,
) -> LalResult<()> {
    debug!("Performing docker permission sanity check");
    let (uid, gid) = permission_sanity_check()?;
    let modified_container: LalResult<Container> = fixup_docker_container(container, uid, gid);

    // Shadow container here
    let container = modified_container.as_ref().unwrap_or(container);

    debug!("Finding home and cwd");
    let home = dirs::home_dir().unwrap(); // crash if no $HOME

    // construct arguments vector
    let mut args: Vec<String> = vec!["run".into(), "--rm".into()];
    for mount in cfg.mounts.clone() {
        debug!(" - mounting {}", mount.src);
        args.push("-v".into());
        let mnt = format!(
                "{}:{}{}",
                mount.src,
                mount.dest,
                if mount.readonly { ":ro" } else { "" }
                );
        args.push(mnt);
    }
    debug!(" - mounting {}", &component_dir.display());
    args.push("-v".into());
    args.push(format!("{}:/home/lal/volume", &component_dir.display()));

    // X11 forwarding
    if modes.x11_forwarding {
        // requires calling `xhost local:docker` first
        args.push("-v".into());
        args.push("/tmp/.X11-unix:/tmp/.X11-unix:ro".into());
        args.push("--env=DISPLAY".into());
        args.push("-v".into());
        // xauth also needed for `ssh -X` through `lal -X`
        args.push(format!("{}/.Xauthority:/home/lal/.Xauthority:ro", home.display()));
        // QT compat
        args.push("--env=QT_X11_NO_MITSHM=1".into());
    }
    if modes.host_networking {
        // also needed for for `ssh -X` into `lal -X`
        args.push("--net=host".into());
    }
    for var in modes.env_vars.clone() {
        args.push(format!("--env={}", var));
    }

    if flags.privileged {
        args.push("--privileged".into())
    }

    args.push("-w".into());
    args.push("/home/lal/volume".into());
    args.push("--user".into());
    args.push(format!("{}:{}", uid, gid));

    // If no command, then override entrypoint to /bin/bash
    // This happens when we use `lal shell` without args
    if command.is_empty() {
        args.push("--entrypoint".into());
        args.push("/bin/bash".into());
    }
    args.push((if flags.interactive { "-it" } else { "-t" }).into());

    args.push(format!("{}:{}", container.name, container.tag));
    for c in command {
        args.push(c);
    }

    // run or print docker command
    if modes.printonly {
        print!("docker");
        for arg in args {
            if arg.contains(' ') {
                // leave quoted args quoted
                print!(" \"{}\"", arg);
            } else {
                print!(" {}", arg);
            }
        }
        println!();
    } else {
        debug!("Entering docker");
        let s = Command::new("docker")
            .args(&args)
            .current_dir(&component_dir)
            .status()?;
        debug!("Exited docker");
        if !s.success() {
            return Err(CliError::SubprocessFailure(s.code().unwrap_or(1001)));
        }
    }
    Ok(())
}
