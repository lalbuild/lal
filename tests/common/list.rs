use std::path::Path;

pub fn list_environments(home: &Path) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
    lal::list::environments(&config)
}

pub fn list_dependencies(component_dir: &Path, core: bool) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::list::dependencies(&manifest, core)
}

pub fn list_configurations(component_dir: &Path) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::list::configurations(&manifest.name.to_string(), &manifest)
}

pub fn list_buildables(component_dir: &Path) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::list::buildables(&manifest)
}
