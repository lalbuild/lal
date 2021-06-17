use std::path::Path;

pub fn stash(component_dir: &Path, backend: &dyn lal::CachedBackend, stash_name: &str) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;

    lal::stash(&component_dir, backend, &manifest, &stash_name)
}
