use std::{ffi::OsStr, path::Path};

pub fn set_environment(
    component_dir: &Path,
    home: &Path,
    sticky: &lal::StickyOptions,
    env_name: &OsStr,
) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
    let env_name = env_name.to_str()
        // Convert Option to Result, until try_trait is stable
        // https://doc.rust-lang.org/std/option/enum.Option.html#impl-Try
        .ok_or(lal::CliError::OptionIsNone)?;

    lal::env::set(&component_dir, &sticky, &config, &env_name)
}

pub fn clear_environment(component_dir: &Path) -> lal::LalResult<()> {
    lal::env::clear(&component_dir)
}
