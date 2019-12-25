extern crate fs_extra;
extern crate lal;
extern crate loggerv;
extern crate tempdir;
extern crate walkdir;

use common::{
    fs_extra::dir::{copy, CopyOptions},
    loggerv::init_with_verbosity,
    tempdir::TempDir,
};

use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::Once,
};

use lal::{BackendConfiguration, Config, LocalBackend};

pub mod build;
pub mod fetch;
pub mod publish;

pub struct TestState {
    pub backend: LocalBackend,
    pub testdir: PathBuf,

    // Keep the tempdir with TestState.
    // The directory will be cleaned with the TestState is `Drop`ed.
    pub tempdir: TempDir,
}

static START: Once = Once::new();

pub fn setup() -> TestState {
    START.call_once(|| {
        env::set_var("SSL_CERT_FILE", "/etc/ssl/certs/ca-certificates.crt");

        // print debug output and greater from lal during tests
        init_with_verbosity(2).expect("Setting up test logging");
    });

    let demo_config = PathBuf::from("./configs/demo.json");
    let testdir = PathBuf::from("./tests");

    // Do all lal tests in a tempdir as it messes with the manifest
    let tempdir = TempDir::new("laltest").unwrap();

    let backend = configure_local_backend(&demo_config, &tempdir.path());

    TestState {
        backend,
        tempdir,
        testdir,
    }
}

fn configure_local_backend(demo_config: &PathBuf, home: &Path) -> LocalBackend {
    let config = Config::read(Some(&home));
    assert!(config.is_err(), "no config at this point");

    let r = lal::configure(
        true,
        false,
        demo_config.as_os_str().to_str().unwrap(),
        Some(&home),
    );
    assert!(r.is_ok(), "configure succeeded");

    let cfg = Config::read(Some(&home));
    assert!(cfg.is_ok(), "config exists now");

    let cfgu = cfg.unwrap();

    match &cfgu.backend {
        &BackendConfiguration::Local(ref local_cfg) => LocalBackend::new(&local_cfg, &cfgu.cache),
        _ => unreachable!(), // demo.json uses local backend
    }
}

// Copies the component to a temporary location for this test
// and sets the working directory to that location
pub fn clone_component_dir(component: &str, state: &TestState) -> PathBuf {
    let copy_options = CopyOptions::new();

    let from = state.testdir.join(component);
    let to = state.tempdir.path().join(component);

    if to.exists() {
        fs::remove_dir_all(&to).expect("clean preexisting content");
    }

    copy(&from, state.tempdir.path(), &copy_options).expect("copy component to tempdir");
    return to;
}

pub fn publish_component(
    state: &TestState,
    env_name: &str,
    component: &str,
    version: &str,
) -> lal::LalResult<PathBuf> {
    let component_dir = clone_component_dir(component, &state);

    fetch::fetch_input(&component_dir, &env_name, &state.backend)?;
    build::build_for_release(&component_dir, &env_name, &state.tempdir.path(), version)?;
    publish::publish_release(&component_dir, &state.backend, &state.tempdir.path())?;

    Ok(component_dir)
}
