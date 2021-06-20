use std::path::Path;

pub fn fetch_input(
    component_dir: &Path,
    env_name: &str,
    backend: &dyn lal::CachedBackend,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    debug!("Component manifest: {:?}", manifest);

    lal::fetch(&component_dir, &manifest, backend, true, &env_name)
}

pub fn fetch_dev_input(
    component_dir: &Path,
    env_name: &str,
    backend: &dyn lal::CachedBackend,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;

    lal::fetch(&component_dir, &manifest, backend, false, &env_name)
}
