use std::{ffi::OsStr, path::Path};

pub fn init(component_dir: &Path, env_name: &OsStr, home: &Path) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
    let env_name = env_name.to_str()
        // Convert Option to Result, until try_trait is stable
        // https://doc.rust-lang.org/std/option/enum.Option.html#impl-Try
        .ok_or(lal::CliError::OptionIsNone)?;

    lal::init(&config, false, &component_dir, &env_name)
}

pub fn init_force(component_dir: &Path, env_name: &OsStr, home: &Path) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
    let env_name = env_name.to_str()
        // Convert Option to Result, until try_trait is stable
        // https://doc.rust-lang.org/std/option/enum.Option.html#impl-Try
        .ok_or(lal::CliError::OptionIsNone)?;

    lal::init(&config, true, &component_dir, &env_name)
}
