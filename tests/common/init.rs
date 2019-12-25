use std::path::Path;

pub fn init(component_dir: &Path, env_name: &str, home: &Path) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
    lal::init(&config, false, &component_dir, &env_name)
}

pub fn init_force(component_dir: &Path, env_name: &str, home: &Path) -> lal::LalResult<()> {
    let config = lal::Config::read(Some(&home))?;
    lal::init(&config, true, &component_dir, &env_name)
}
