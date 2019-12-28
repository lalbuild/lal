use common::*;
use lal::Backend;
use parameterized_macro::parameterized;
use std::{fs, path::Path};

#[parameterized(env_name = {"default", "alpine"})]
pub fn test_configure_backend(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    assert_eq!(
        fs::canonicalize(Path::new(&state.backend.get_cache_dir())).unwrap(),
        fs::canonicalize(&state.tempdir.path().join(".lal/cache")).unwrap(),
    );
}
