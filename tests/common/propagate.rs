use std::path::Path;

pub fn compute(component_dir: &Path, leaf: &str) -> lal::LalResult<lal::propagate::UpdateSequence> {
    let manifest = lal::Manifest::read(&component_dir)?;
    let lockfile = lal::Lockfile::default()
        .set_name(&manifest.name)
        .populate_from_input(&component_dir)?;

    lal::propagate::compute(&lockfile, leaf)
}

pub fn print(component_dir: &Path, leaf: &str) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::propagate::print(&component_dir, &manifest, leaf, false)
}

pub fn print_json(component_dir: &Path, leaf: &str) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::propagate::print(&component_dir, &manifest, leaf, true)
}
