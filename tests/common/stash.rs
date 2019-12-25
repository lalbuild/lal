use std::path::Path;

pub fn stash<T: lal::CachedBackend + lal::Backend>(
    component_dir: &Path,
    backend: &T,
    stash_name: &str,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;

    lal::stash(&component_dir, backend, &manifest, &stash_name)
}
