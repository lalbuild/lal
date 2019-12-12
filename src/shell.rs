use std::{path::Path, vec::Vec};

use super::{docker_run, native_run, CliError, Config, DockerRunFlags, Environment, LalResult, ShellModes};

/// Runs an arbitrary command in the configured environment
/// delegating to docker if needed.
///
/// This is the most general function, used by both `lal build` and `lal shell`.
pub fn run(
    cfg: &Config,
    environment: &Environment,
    command: Vec<String>,
    flags: &DockerRunFlags,
    modes: &ShellModes,
    component_dir: &Path,
) -> LalResult<()> {
    match environment {
        Environment::Container(container) => {
            docker_run(&cfg, &container, command, &flags, &modes, &component_dir)
        }
        Environment::None => native_run(command, &component_dir),
    }
}


/// Mounts and enters `.` in an interactive bash shell using the configured container.
///
/// If a command vector is given, this is called non-interactively instead of /bin/bash
/// You can thus do `lal shell ./BUILD target` or ``lal shell bash -c "cmd1; cmd2"`
pub fn shell(
    cfg: &Config,
    environment: &Environment,
    modes: &ShellModes,
    cmd: Option<Vec<&str>>,
    privileged: bool,
    component_dir: &Path,
) -> LalResult<()> {
    let flags = DockerRunFlags {
        interactive: cmd.is_none() || cfg.interactive,
        privileged,
    };

    let mut command = vec![];
    if let Some(cmdu) = cmd {
        for c in cmdu {
            command.push(c.to_string())
        }
    }

    match environment {
        Environment::Container(container) => {
            if !modes.printonly {
                info!("Entering {}", container);
            }

            docker_run(cfg, container, command, &flags, modes, &component_dir)
        }
        Environment::None => {
            if command.is_empty() {
                command.push("bash".into());
            }

            native_run(command, &component_dir)
        }
    }
}

/// Runs a script in `.lal/scripts/` with supplied arguments in a docker shell
///
/// This is a convenience helper for running things that aren't builds.
/// E.g. `lal run my-large-test RUNONLY=foo`
pub fn script(
    cfg: &Config,
    environment: &Environment,
    name: &str,
    args: Vec<&str>,
    modes: &ShellModes,
    privileged: bool,
    component_dir: &Path,
) -> LalResult<()> {
    let pth = Path::new(".lal").join("scripts").join(&name);
    debug!("pth: {}", &pth.display());
    if !component_dir.join(&pth).exists() {
        return Err(CliError::MissingScript(name.into()));
    }

    // Simply run the script by adding on the arguments
    let command = vec![
        "bash".into(),
        "-c".into(),
        format!("source {}; main {}", &pth.display(), args.join(" ")),
    ];

    debug!("script command: {:?}", command);
    match environment {
        Environment::Container(container) => {
            let flags = DockerRunFlags {
                interactive: cfg.interactive,
                privileged,
            };

            Ok(docker_run(
                cfg,
                container,
                command,
                &flags,
                modes,
                &component_dir,
            )?)
        }
        Environment::None => native_run(command, &component_dir),
    }
}
