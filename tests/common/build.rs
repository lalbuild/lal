use std::path::Path;

pub fn options(home: Option<&Path>, env_name: &str, manifest: &lal::Manifest) -> lal::LalResult<lal::BuildOptions> {
    let config = lal::Config::read(home)?;
    debug!("options: config: {:#?}", config);

    let environment = manifest.get_environment(env_name)
        .or(config.get_environment(env_name))?;
    debug!("options: environment: {}", environment);

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
    let manifest = lal::Manifest::read(&component_dir)?;
    let mut build_opts = options(Some(&home), &env_name, &manifest)?;
    build_opts.version = Some(version.to_string());

    build_with_options(&component_dir, &manifest, &env_name, &home, &build_opts)
}

pub fn build_with_options(
    component_dir: &Path,
    manifest: &lal::Manifest,
    env_name: &str,
    home: &Path,
    build_opts: &lal::BuildOptions,
) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
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
