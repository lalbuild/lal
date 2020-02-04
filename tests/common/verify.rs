use std::{ffi::OsStr, path::Path};

pub fn verify(component_dir: &Path, env_name: &OsStr, simple: bool) -> lal::LalResult<lal::Manifest> {
    let env_name = env_name.to_str()
        .ok_or(lal::CliError::OptionIsNone)?;

    let manifest = lal::Manifest::read(&component_dir)?;
    lal::verify(&component_dir, &manifest, &env_name, simple)?;

    Ok(manifest)
}
