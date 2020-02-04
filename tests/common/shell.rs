use std::{ffi::OsStr, path::Path};

pub fn run(env_name: &OsStr, home: &Path, component_dir: &Path, command_args: Vec<&str>) -> lal::LalResult<()> {
    let cfg = lal::Config::read(Some(&home))?;
    let environment = cfg.get_environment(&env_name)?;
    let modes = lal::ShellModes::default();

    let mut args = Vec::<String>::new();
    for arg in &command_args {
        args.push(arg.to_string());
    }

    lal::run(
        &cfg,
        &environment,
        args,
        &lal::DockerRunFlags::default(),
        &modes,
        &component_dir,
    )
}

pub fn run_script(
    env_name: &OsStr,
    home: &Path,
    component_dir: &Path,
    script_name: &str,
    script_args: Vec<&str>,
) -> lal::LalResult<()> {
    let cfg = lal::Config::read(Some(&home))?;
    let environment = cfg.get_environment(env_name)?;
    let modes = lal::ShellModes::default();

    lal::script(
        &cfg,
        &environment,
        &script_name,
        script_args,
        &modes,
        false,
        &component_dir,
    )
}
