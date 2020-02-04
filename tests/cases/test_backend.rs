use crate::common::*;
use lal::Backend;
use std::{fs, path::Path};

#[test]
pub fn test_configure_backend() {
    let state = setup();

    assert_eq!(
        fs::canonicalize(Path::new(&state.backend.get_cache_dir())).unwrap(),
        fs::canonicalize(&state.tempdir.path().join(".lal/cache")).unwrap(),
    );
}
