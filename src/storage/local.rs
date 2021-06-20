#![allow(missing_docs)]

use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    str::FromStr,
    vec::Vec,
};

use crate::core::{config_dir, ensure_dir_exists_fresh, CliError, LalResult};

/// LocalBackend configuration options (currently none)
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LocalConfig {}

use super::{Backend, Component};

/// Artifact storage on the local machine
pub struct LocalBackend {
    /// Local config
    pub config: LocalConfig,
    /// Cache directory
    pub cache: PathBuf,
}

impl LocalBackend {
    pub fn new(cfg: &LocalConfig, cache: &Path) -> Self {
        match fs::create_dir(&cache) {
            Ok(()) => Ok(()),
            Err(ref e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(e) => Err(e),
        }
        .unwrap();

        LocalBackend {
            config: cfg.clone(),
            cache: cache.to_path_buf(),
        }
    }
}

/// Artifact backend trait for `LocalBackend`
///
/// This is intended to be used by the caching trait `CachedBackend`, but for
/// specific low-level use cases, these methods can be used directly.
impl Backend for LocalBackend {
    fn get_versions(&self, name: &str, loc: &str) -> LalResult<Vec<u32>> {
        let tar_dir = format!("{}/environments/{}/{}/", self.cache.display(), loc, name);
        let dentries = fs::read_dir(config_dir(None).join(tar_dir));
        let mut versions = vec![];
        for entry in dentries? {
            let path = entry?;
            if let Some(filename) = path.file_name().to_str() {
                if let Ok(version) = u32::from_str(filename) {
                    versions.push(version);
                }
            }
        }
        Ok(versions)
    }

    fn get_latest_version(&self, name: &str, loc: &str) -> LalResult<u32> {
        if let Some(&last) = self.get_versions(name, loc)?.last() {
            return Ok(last);
        }
        Err(CliError::BackendFailure(
            "No versions found on local storage".into(),
        ))
    }

    fn get_component_info(&self, name: &str, version: Option<u32>, loc: &str) -> LalResult<Component> {
        info!("get_component_info: {} {:?} {}", name, version, loc);

        let v = if let Some(ver) = version {
            ver
        } else {
            self.get_latest_version(name, loc)?
        };
        let loc = format!(
            "{}/environments/{}/{}/{}/{}.tar.gz",
            self.cache.display(),
            loc,
            name,
            v,
            name
        );
        Ok(Component {
            name: name.into(),
            version: v,
            location: loc,
        })
    }

    fn publish_artifact(
        &self,
        home: Option<&Path>,
        component_dir: &Path,
        name: &str,
        version: u32,
        env: &str,
    ) -> LalResult<()> {
        // this fn basically assumes all the sanity checks have been performed
        // files must exist and lockfile must be sensible
        let artifactdir = component_dir.join("./ARTIFACT");
        let tarball = artifactdir.join(format!("{}.tar.gz", name));
        let lockfile = artifactdir.join("lockfile.json");

        // prefix with environment
        let tar_dir = format!(
            "{}/environments/{}/{}/{}/",
            self.cache.display(),
            env,
            name,
            version
        );
        let tar_path = format!(
            "{}/environments/{}/{}/{}/{}.tar.gz",
            self.cache.display(),
            env,
            name,
            version,
            name
        );
        let lock_path = format!(
            "{}/environments/{}/{}/{}/lockfile.json",
            self.cache.display(),
            env,
            name,
            version
        );

        let full_tar_dir = config_dir(home).join(tar_dir);
        ensure_dir_exists_fresh(&full_tar_dir)?;

        fs::copy(tarball, config_dir(home).join(tar_path))?;
        fs::copy(lockfile, config_dir(home).join(lock_path))?;

        Ok(())
    }

    fn get_cache_dir(&self) -> PathBuf {
        self.cache.clone()
    }

    fn raw_fetch(&self, src: &str, dest: &Path) -> LalResult<()> {
        debug!("raw fetch {} -> {}", src, dest.display());
        fs::copy(src, dest)?;
        Ok(())
    }
}
