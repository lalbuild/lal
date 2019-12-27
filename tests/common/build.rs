use std::path::Path;

pub fn options(home: Option<&Path>, env_name: &str) -> lal::LalResult<lal::BuildOptions> {
    let config = lal::Config::read(home)?;
    let environment = config.get_environment(env_name.to_string())?;

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
    env_name: &str,
    home: &Path,
    version: &str,
) -> lal::LalResult<()> {
    let mut build_opts = options(Some(&home), &env_name)?;
    build_opts.version = Some(version.to_string());

    build_with_options(&component_dir, &env_name, &home, &build_opts)
}

pub fn build_with_options(
    component_dir: &Path,
    env_name: &str,
    home: &Path,
    build_opts: &lal::BuildOptions,
) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
    let manifest = lal::Manifest::read(&component_dir)?;
    let modes = lal::ShellModes::default();

    lal::build(
        &component_dir,
        &config,
        &manifest,
        &build_opts,
        env_name.to_string(),
        modes,
    )
    .map_err(|err| {
        error!("build: {:?}", err);
        err
    })
}

pub fn build_with_options_and_modes(
    component_dir: &Path,
    env_name: &str,
    home: &Path,
    build_opts: &lal::BuildOptions,
    modes: lal::ShellModes,
) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
    let manifest = lal::Manifest::read(&component_dir)?;

    lal::build(
        &component_dir,
        &config,
        &manifest,
        &build_opts,
        env_name.to_string(),
        modes,
    )
}
