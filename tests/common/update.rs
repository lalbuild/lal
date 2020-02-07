use std::path::Path;

pub fn update(
    component_dir: &Path,
    env_name: &str,
    backend: &dyn lal::CachedBackend,
    components: Vec<&str>,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;

    let mut dependencies = Vec::<String>::new();
    for component in &components {
        dependencies.push(component.to_string());
    }

    lal::update(
        &component_dir,
        &manifest,
        backend,
        dependencies,
        false,
        false,
        &env_name,
    )
}

pub fn update_all(
    component_dir: &Path,
    env_name: &str,
    backend: &dyn lal::CachedBackend,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::update_all(&component_dir, &manifest, backend, false, false, &env_name)
}

pub fn update_with_save(
    component_dir: &Path,
    env_name: &str,
    backend: &dyn lal::CachedBackend,
    components: Vec<&str>,
    save: bool,
    savedev: bool,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;

    let mut dependencies = Vec::<String>::new();
    for component in &components {
        dependencies.push(component.to_string());
    }

    lal::update(
        &component_dir,
        &manifest,
        backend,
        dependencies,
        save,
        savedev,
        &env_name,
    )
}

pub fn update_all_with_save(
    component_dir: &Path,
    env_name: &str,
    backend: &dyn lal::CachedBackend,
    save: bool,
    savedev: bool,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::update_all(&component_dir, &manifest, backend, save, savedev, &env_name)
}
