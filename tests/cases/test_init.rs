use crate::common::*;
use parameterized_macro::parameterized;
use std::{ffi::OsStr, fs};

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_init(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = state.tempdir.path().join("new_component");
    fs::create_dir(&component_dir).expect("create new_component dir");

    // Init a new component from an empty directory
    let r = init::init(&component_dir, &env_name, &state.tempdir.path());
    assert!(r.is_ok(), "new component created");

    let manifest = verify::verify(&component_dir, &env_name, false).expect("verify manifest");
    assert_eq!(manifest.name, "new_component".to_string());
    assert_eq!(manifest.environment, env_name.to_string_lossy().to_string());
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_reinit_without_force(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = state.tempdir.path().join("new_component");
    fs::create_dir(&component_dir).expect("create new_component dir");

    // Initial init
    let r = init::init(&component_dir, &env_name, &state.tempdir.path());
    assert!(r.is_ok(), "new component created");

    // Reinit fails, can't init twice
    let r = init::init(&component_dir, &env_name, &state.tempdir.path());
    assert!(r.is_err(), "can't init new component again");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_reinit_with_force(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = state.tempdir.path().join("new_component");
    fs::create_dir(&component_dir).expect("create new_component");

    // Initial init
    let r = init::init(&component_dir, &env_name, &state.tempdir.path());
    assert!(r.is_ok(), "init new component");

    // Force reinit
    let r = init::init_force(&component_dir, &env_name, &state.tempdir.path());
    assert!(r.is_ok(), "reinit new component with force");
}

#[test]
fn test_init_with_invalid_env_name() {
    let state = setup();

    let component_dir = state.tempdir.path().join("new_component");
    fs::create_dir(&component_dir).expect("create new_component");

    // Init, with an environment not in ~/.lal.config
    let r = init::init(&component_dir, OsStr::new("nonexistant"), &state.tempdir.path());
    assert!(r.is_err(), "can't init with a nonexistant environment");
}
