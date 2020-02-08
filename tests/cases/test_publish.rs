use crate::common::*;
use parameterized_macro::parameterized;
use std::{ffi::OsStr, path::Path};
use lal::Backend;

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_publish(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // publish heylib, a component without a dependency
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    let r = build::build_for_release(&component_dir, &env_name, &state.tempdir.path(), "1");
    assert!(r.is_ok(), "built heylib release");

    let r = publish::publish_release(&component_dir, &state.backend, &state.tempdir.path());
    assert!(r.is_ok(), "published heylib=1 release");

    // Check for published artifact in cache_dir
    let cache_dir = state.backend.get_cache_dir();
    let artifact = Path::new(&cache_dir)
        .join("environments")
        .join(&env_name)
        .join("heylib/1/heylib.tar.gz");

    assert!(artifact.exists(), "published artifact to local storage");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_publish_without_release(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // publish heylib, a component without a dependency
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    let mut build_opts = build::options(Some(&state.tempdir.path()), &env_name).expect("build options");
    build_opts.version = Some("1".into());
    build_opts.release = false;

    let r = build::build_with_options(&component_dir, &env_name, &state.tempdir.path(), &build_opts);
    assert!(r.is_ok(), "built heylib without release");

    let r = publish::publish_release(&component_dir, &state.backend, &state.tempdir.path());
    assert!(r.is_err(), "can't publish non-release build");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_publish_without_version(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // publish heylib, a component without a dependency
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    let mut build_opts = build::options(Some(&state.tempdir.path()), &env_name).expect("build options");
    build_opts.version = None;
    build_opts.release = true;

    let r = build::build_with_options(&component_dir, &env_name, &state.tempdir.path(), &build_opts);
    assert!(r.is_ok(), "built heylib without version");

    let r = publish::publish_release(&component_dir, &state.backend, &state.tempdir.path());
    assert!(r.is_err(), "can't publish without version");
}
