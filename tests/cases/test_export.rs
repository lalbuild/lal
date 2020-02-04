use crate::common::*;
use parameterized_macro::parameterized;
use std::{ffi::OsStr, fs};

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
pub fn test_export_published_version(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1")
        .expect("published heylib=1 helloworld=1");

    let export_dir = &state.tempdir.path().join("export");
    assert!(fs::create_dir(&export_dir).is_ok(), "create export_dir");

    let r = lal::export(&state.backend, "heylib=1", &export_dir, env_name.to_str());
    assert!(r.is_ok(), "exported heylib=1");
    assert!(export_dir.join("heylib.tar.gz").is_file(), "heylib=1 export ok");

    let r = lal::export(&state.backend, "hello=1", &export_dir, env_name.to_str());
    assert!(r.is_ok(), "exported hello=1");
    assert!(export_dir.join("hello.tar.gz").is_file(), "hello=1 export ok");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
pub fn test_export_latest_published(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1")
        .expect("published heylib=1 helloworld=1");

    let export_dir = &state.tempdir.path().join("export");
    assert!(fs::create_dir(&export_dir).is_ok(), "create export_dir");

    let r = lal::export(&state.backend, "heylib", &export_dir, env_name.to_str());
    assert!(r.is_ok(), "exported heylib");
    assert!(export_dir.join("heylib.tar.gz").is_file(), "heylib export ok");

    let r = lal::export(&state.backend, "hello", &export_dir, env_name.to_str());
    assert!(r.is_ok(), "exported hello");
    assert!(export_dir.join("hello.tar.gz").is_file(), "hello export ok");
}
