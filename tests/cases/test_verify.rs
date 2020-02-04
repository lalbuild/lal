use crate::common::*;
use parameterized_macro::parameterized;
use std::ffi::OsStr;

#[parameterized(
    env_name = {OsStr::new("default"), OsStr::new("default"), OsStr::new("alpine"), OsStr::new("alpine")},
    simple = {true, false, true, false},
)]
fn test_verify_no_deps(env_name: &OsStr, simple: bool) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed core dependencies");

    let r = verify::verify(&component_dir, &env_name, simple);
    assert!(r.is_ok(), "verified INPUT consistency");
}

#[parameterized(
    env_name = {OsStr::new("default"), OsStr::new("default"), OsStr::new("alpine"), OsStr::new("alpine")},
    simple = {true, false, true, false},
)]
fn test_verify_with_deps(env_name: &OsStr, simple: bool) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // helloworld component depends on heylib
    publish_component(&state, &env_name, "heylib", "1").expect("published heylib=1");

    let component_dir = clone_component_dir("helloworld", &state);

    let r = verify::verify(&component_dir, &env_name, simple);
    assert!(r.is_err(), "verify fails without INPUTs");

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed core dependencies");

    let r = verify::verify(&component_dir, &env_name, simple);
    assert!(r.is_ok(), "verified INPUT consistency");
}

#[parameterized(
    env_name = {OsStr::new("default"), OsStr::new("default"), OsStr::new("alpine"), OsStr::new("alpine")},
    simple = {true, false, true, false},
)]
fn test_verify_with_deps_in_wrong_env(env_name: &OsStr, simple: bool) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // helloworld component depends on heylib
    publish_component(&state, &env_name, "heylib", "1").expect("published heylib=1");

    let component_dir = clone_component_dir("helloworld", &state);
    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed core dependencies");

    // Succeeds with the correct environment
    let r = verify::verify(&component_dir, &env_name, simple);
    assert!(r.is_ok(), "verified INPUT consistency");

    let r = verify::verify(&component_dir, OsStr::new("xenial"), simple);
    assert!(
        r.is_err(),
        "verify fails when checking with inconsistent environments"
    );
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_verify_with_stashed_deps(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Initial build to generate a stashed component
    stash_component(&state, &env_name, "heylib", "blah").expect("stashed heylib=blah");

    // Main build, with stashed depencency
    let component_dir = clone_component_dir("helloworld", &state);

    let r = verify::verify(&component_dir, &env_name, false);
    assert!(r.is_err(), "verify fails without dependencies");
    let r = verify::verify(&component_dir, &env_name, true);
    assert!(r.is_err(), "simple verify fails without dependencies");

    let r = update::update(&component_dir, &env_name, &state.backend, vec!["heylib=blah"]);
    assert!(r.is_ok(), "using stashed dependency");

    let r = verify::verify(&component_dir, &env_name, false);
    assert!(r.is_err(), "verify fails with stashed dependencies");
    let r = verify::verify(&component_dir, &env_name, true);
    assert!(
        r.is_ok(),
        "allow stashed versions with the simpler verify algorithm"
    );
}
