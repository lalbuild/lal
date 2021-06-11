use std::path::{Component, Path};

use super::{CliError, Config, LalResult};
use crate::core::manifest::*;

/// Generates a blank manifest in the current directory
///
/// This will use the directory name as the assumed default component name
/// Then fill in the blanks as best as possible.
///
/// The function will not overwrite an existing `manifest.json`,
/// unless the `force` bool is set.
pub fn init(cfg: &Config, force: bool, component_dir: &Path, env: &str) -> LalResult<()> {
    cfg.get_environment(env)?;

    let last_comp: Component<'_> = component_dir.components().last().unwrap();
    let dirname = last_comp.as_os_str().to_str().unwrap();

    let mpath = ManifestLocation::identify(&component_dir.to_path_buf());
    if !force && mpath.is_ok() {
        return Err(CliError::ManifestExists);
    }

    // we are allowed to overwrite or write a new manifest if we are here
    // always create new manifests in new default location
    create_lal_subdir(&component_dir.to_path_buf())?; // create the `.lal` subdir if it's not there already
    Manifest::new(
        dirname,
        env,
        ManifestLocation::default().as_path(&component_dir.to_path_buf()),
    )
    .write()?;

    // if the manifest already existed, warn about this now being placed elsewhere
    if let Ok(ManifestLocation::RepoRoot) = mpath {
        warn!("Created manifest in new location under .lal");
        warn!("Please delete the old manifest - it will not be read anymore");
    }

    Ok(())
}
