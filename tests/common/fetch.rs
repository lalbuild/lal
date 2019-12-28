use std::path::Path;

pub fn fetch_input<T: lal::CachedBackend + lal::Backend>(
    component_dir: &Path,
    env_name: &str,
    backend: &T,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;

    lal::fetch(&component_dir, &manifest, backend, true, &env_name)
}

pub fn fetch_dev_input<T: lal::CachedBackend + lal::Backend>(
    component_dir: &Path,
    env_name: &str,
    backend: &T,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;

    lal::fetch(&component_dir, &manifest, backend, false, &env_name)
}
