use crate::common::*;
use parameterized_macro::parameterized;
use std::ffi::OsStr;

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_fetch_no_deps(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed core dependencies");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_fetch_no_dev_deps(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_dev_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed dev dependencies");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_fetch_with_deps(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // heylib component is a dependency, needs to be publishd first
    publish_component(&state, &env_name, "heylib", "1").expect("publish heylib=1");

    // helloworld depends on heylib
    let component_dir = clone_component_dir("helloworld", &state);
    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed helloworld dependencies");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_fetch_with_dev_deps(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // heylib component is a dependency, needs to be published first
    publish_component(&state, &env_name, "heylib", "1").expect("publish heylib=1");

    // helloworld depends on heylib
    let component_dir = clone_component_dir("helloworld", &state);
    let r = fetch::fetch_dev_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed helloworld dev dependencies");
}
