use std::{ffi::OsStr, path::Path};

pub fn update<T: lal::CachedBackend + lal::Backend>(
    component_dir: &Path,
    env_name: &OsStr,
    backend: &T,
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

pub fn update_all<T: lal::CachedBackend + lal::Backend>(
    component_dir: &Path,
    env_name: &OsStr,
    backend: &T,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::update_all(&component_dir, &manifest, backend, false, false, &env_name)
}

pub fn update_with_save<T: lal::CachedBackend + lal::Backend>(
    component_dir: &Path,
    env_name: &OsStr,
    backend: &T,
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

pub fn update_all_with_save<T: lal::CachedBackend + lal::Backend>(
    component_dir: &Path,
    env_name: &OsStr,
    backend: &T,
    save: bool,
    savedev: bool,
) -> lal::LalResult<()> {
    let manifest = lal::Manifest::read(&component_dir)?;
    lal::update_all(&component_dir, &manifest, backend, save, savedev, &env_name)
}
