use std::path::Path;

// Need both the struct and the trait
use super::{CliError, LalResult, Lockfile};
use crate::storage::CachedBackend;

/// Publish a release build to the storage backend
///
/// Meant to be done after a `lal build -r <component>`
/// and requires publish credentials in the local `Config`.
pub fn publish(
    home: Option<&Path>,
    component_dir: &Path,
    name: &str,
    backend: &dyn CachedBackend,
) -> LalResult<()> {
    let artdir = component_dir.join("./ARTIFACT");
    let tarball = artdir.join(format!("{}.tar.gz", name));
    if !artdir.is_dir() || !tarball.exists() {
        warn!("Missing: {}", tarball.display());
        return Err(CliError::MissingReleaseBuild);
    }

    let lock = Lockfile::release_build(&component_dir)?;

    let version = lock.version.parse::<u32>().map_err(|e| {
        error!("Release build not done --with-version=$BUILD_VERSION");
        debug!("Error: {}", e);
        CliError::MissingReleaseBuild
    })?;

    if lock.sha.is_none() {
        warn!("Release build not done --with-sha=$(git rev-parse HEAD)");
    }

    // always publish to the environment in the lockfile
    let envname = lock.envname;

    info!("Publishing {}={} to {}", name, version, envname);
    backend.publish_artifact(home, &component_dir, name, version, &envname)?;

    Ok(())
}
