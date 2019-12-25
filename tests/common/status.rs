use std::path::Path;

pub fn status(component_dir: &Path) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::status(&component_dir, &manifest, false, false, false)
}

pub fn full_status(component_dir: &Path) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::status(&component_dir, &manifest, true, false, false)
}

pub fn full_descriptive_status(component_dir: &Path) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::status(&component_dir, &manifest, true, true, true)
}
