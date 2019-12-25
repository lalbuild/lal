use std::path::Path;

pub fn publish_release<T: lal::CachedBackend + lal::Backend>(
    component_dir: &Path,
    backend: &T,
    home: &Path,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;

    lal::publish(Some(&home), &component_dir, &manifest.name, backend)
}
