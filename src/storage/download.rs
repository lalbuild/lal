use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    core::{output, CliError, LalResult},
    storage::{Backend, CachedBackend, Component},
};

fn is_cached(backend: &dyn Backend, name: &str, version: u32, env: &str) -> bool {
    get_cache_dir(backend, name, version, env).is_dir()
}

fn get_cache_dir(backend: &dyn Backend, name: &str, version: u32, env: &str) -> PathBuf {
    let cache = backend.get_cache_dir();
    Path::new(&cache)
        .join("environments")
        .join(env)
        .join(name)
        .join(version.to_string())
}

fn stored_tarball_location(
    backend: &dyn Backend,
    name: &str,
    version: u32,
    env: &str,
) -> Result<PathBuf, CliError> {
    // 1. mkdir -p cacheDir/$name/$version
    let destdir = get_cache_dir(backend, name, version, env);
    if !destdir.is_dir() {
        fs::create_dir_all(&destdir)?;
    }
    // 2. stuff $PWD/$name.tar.gz in there
    let tarname = [name, ".tar.gz"].concat();
    let dest = Path::new(&destdir).join(&tarname);

    Ok(dest)
}

// helper for the unpack_ functions
fn extract_tarball_to_input(tarname: PathBuf, component_dir: &Path, component: &str) -> LalResult<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let extract_path = component_dir.join("INPUT").join(component);
    let _ = fs::remove_dir_all(&extract_path); // remove current dir if exists
    fs::create_dir_all(&extract_path)?;
    debug!("extract path: {}", extract_path.display());

    // Open file, conditionally wrap a progress bar around the file reading
    if cfg!(feature = "progress") {
        #[cfg(feature = "progress")]
        {
            use super::progress::ProgressReader;
            let data = fs::File::open(tarname)?;
            let progdata = ProgressReader::new(data)?;
            let decompressed = GzDecoder::new(progdata)?; // decoder reads data (proxied)
            let mut archive = Archive::new(decompressed); // Archive reads decoded
            archive.unpack(&extract_path)?;
        }
    } else {
        let data = fs::File::open(tarname)?;
        let decompressed = GzDecoder::new(data)?; // decoder reads data
        let mut archive = Archive::new(decompressed); // Archive reads decoded
        archive.unpack(&extract_path)?;
    };

    debug!("---");
    Ok(())
}

/// Cacheable trait implemented for all Backends.
///
/// As long as we have the Backend trait implemented, we can add a caching layer
/// around this, which implements the basic compression ops and file gymnastics.
///
/// Most subcommands should be OK with just using this trait rather than using
/// `Backend` directly as this does the stuff you normally would want done.
impl<T: Backend> CachedBackend for T {
    /// Get the latest versions of a component across all supported environments
    ///
    /// Because the versions have to be available in all environments, these numbers may
    /// not contain the highest numbers available on specific environments.
    fn get_latest_supported_versions(&self, name: &str, environments: Vec<String>) -> LalResult<Vec<u32>> {
        use std::collections::BTreeSet;
        let mut result = BTreeSet::new();
        let mut first_pass = true;
        for e in environments {
            let eres: BTreeSet<_> = self.get_versions(name, &e)?.into_iter().take(100).collect();
            info!("Last versions for {} in {} env is {:?}", name, e, eres);
            if first_pass {
                // if first pass, can't take intersection with something empty, start with first result
                result = eres;
                first_pass = false;
            } else {
                result = result.clone().intersection(&eres).cloned().collect();
            }
        }
        debug!("Intersection of allowed versions {:?}", result);
        Ok(result.into_iter().collect())
    }

    /// Locate a proper component, downloading it and caching if necessary
    fn retrieve_published_component(
        &self,
        name: &str,
        version: Option<u32>,
        env: &str,
    ) -> LalResult<(PathBuf, Component)> {
        trace!("Locate component {}", name);

        let component = self.get_component_info(name, version, env)?;

        if !is_cached(self, &component.name, component.version, env) {
            // download to PWD then move it to stash immediately
            let tarball_location = stored_tarball_location(self, name, component.version, env)?;
            self.raw_fetch(&component.location, &tarball_location)?;
        }
        assert!(
            is_cached(self, &component.name, component.version, env),
            "cached component"
        );

        trace!("Fetching {} from cache", name);
        let tarname =
            get_cache_dir(self, &component.name, component.version, env).join(format!("{}.tar.gz", name));
        Ok((tarname, component))
    }

    // basic functionality for `fetch`/`update`
    fn unpack_published_component(
        &self,
        component_dir: &Path,
        name: &str,
        version: Option<u32>,
        env: &str,
    ) -> LalResult<Component> {
        let (tarname, component) = self.retrieve_published_component(name, version, env)?;

        debug!(
            "Unpacking tarball {} for {}",
            tarname.to_str().unwrap(),
            component.name
        );
        extract_tarball_to_input(tarname, &component_dir, name)?;

        Ok(component)
    }

    /// helper for `update`
    fn unpack_stashed_component(&self, component_dir: &Path, name: &str, code: &str) -> LalResult<()> {
        let tarpath = self.retrieve_stashed_component(name, code)?;

        extract_tarball_to_input(tarpath, &component_dir, name)?;
        Ok(())
    }

    /// helper for unpack_, `export`
    fn retrieve_stashed_component(&self, name: &str, code: &str) -> LalResult<PathBuf> {
        let tarpath = Path::new(&self.get_cache_dir())
            .join("stash")
            .join(name)
            .join(code)
            .join(format!("{}.tar.gz", name));
        if !tarpath.is_file() {
            return Err(CliError::MissingStashArtifact(format!("{}/{}", name, code)));
        }
        Ok(tarpath)
    }

    // helper for `stash`
    fn stash_output(&self, component_dir: &Path, name: &str, code: &str) -> LalResult<()> {
        let destdir = Path::new(&self.get_cache_dir())
            .join("stash")
            .join(name)
            .join(code);
        debug!("Creating {:?}", destdir);
        fs::create_dir_all(&destdir)?;

        // Tar it straight into destination
        output::tar(&component_dir, &destdir.join(format!("{}.tar.gz", name)))?;

        // Copy the lockfile there for users inspecting the stashed folder
        // NB: this is not really needed, as it's included in the tarball anyway
        fs::copy(
            &component_dir.join("OUTPUT/lockfile.json"),
            destdir.join("lockfile.json"),
        )?;
        Ok(())
    }
}
