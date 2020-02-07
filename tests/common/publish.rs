use std::path::Path;

pub fn publish_release(
    component_dir: &Path,
    backend: &dyn lal::CachedBackend,
    home: &Path,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;

    lal::publish(Some(&home), &component_dir, &manifest.name, backend)
}
