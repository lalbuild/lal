use std::{ffi::OsStr, path::Path};

pub fn options(home: Option<&Path>, env_name: &OsStr) -> lal::LalResult<lal::BuildOptions> {
    let config = lal::Config::read(home)?;
    let environment = config.get_environment(&env_name)?;

    Ok(lal::BuildOptions {
        name: None,
        configuration: Some("release".into()),
        environment: environment,
        release: true,
        version: None,
        sha: None,
        force: false,
        simple_verify: false,
    })
}

pub fn build_for_release(
    component_dir: &Path,
    env_name: &OsStr,
    home: &Path,
    version: &str,
) -> lal::LalResult<()> {
    let mut build_opts = options(Some(&home), &env_name)?;
    build_opts.version = Some(version.to_string());

    build_with_options(&component_dir, &env_name, &home, &build_opts)
}

pub fn build_with_options(
    component_dir: &Path,
    env_name: &OsStr,
    home: &Path,
    build_opts: &lal::BuildOptions,
) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
    let manifest = lal::Manifest::read(&component_dir)?;
    let modes = lal::ShellModes::default();
    let env_name = env_name.to_str()
        // Convert Option to Result, until try_trait is stable
        // https://doc.rust-lang.org/std/option/enum.Option.html#impl-Try
        .ok_or(lal::CliError::OptionIsNone)?;

    lal::build(
        &component_dir,
        &config,
        &manifest,
        &build_opts,
        env_name.to_string(),
        modes,
    )
}

pub fn build_with_options_and_modes(
    component_dir: &Path,
    env_name: &OsStr,
    home: &Path,
    build_opts: &lal::BuildOptions,
    modes: lal::ShellModes,
) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
    let manifest = lal::Manifest::read(&component_dir)?;
    let env_name = env_name.to_str()
        // Convert Option to Result, until try_trait is stable
        // https://doc.rust-lang.org/std/option/enum.Option.html#impl-Try
        .ok_or(lal::CliError::OptionIsNone)?;

    lal::build(
        &component_dir,
        &config,
        &manifest,
        &build_opts,
        env_name.to_string(),
        modes,
    )
}
