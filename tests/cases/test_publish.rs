use crate::common::*;
use parameterized_macro::parameterized;
use std::path::Path;

use lal::Backend;

#[parameterized(env_name = {"default", "alpine"})]
fn test_publish(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // publish heylib, a component without a dependency
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies: {:?}", r);

    let r = build::build_for_release(&component_dir, &env_name, &state.tempdir.path(), "1");
    assert!(r.is_ok(), "built heylib release: {:?}", r);

    let r = publish::publish_release(&component_dir, &state.backend, &state.tempdir.path());
    assert!(r.is_ok(), "published heylib=1 release: {:?}", r);

    // Check for published artifact in cache_dir
    let cache_dir = state.backend.get_cache_dir();
    let artifact = Path::new(&cache_dir)
        .join("environments")
        .join(&env_name)
        .join("heylib/1/heylib.tar.gz");

    assert!(
        artifact.exists(),
        "published artifact to local storage: {:?}",
        artifact
    );
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_publish_without_release(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // publish heylib, a component without a dependency
    let component_dir = clone_component_dir("heylib", &state);
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies: {:?}", r);

    let mut build_opts = build::options(Some(&state.tempdir.path()), &env_name, &manifest).expect("build options");
    build_opts.version = Some("1".into());
    build_opts.release = false;

    let r = build::build_with_options(&component_dir, &manifest, &env_name, &state.tempdir.path(), &build_opts);
    assert!(r.is_ok(), "built heylib without release: {:?}", r);

    let r = publish::publish_release(&component_dir, &state.backend, &state.tempdir.path());
    assert!(r.is_err(), "can't publish non-release build: {:?}", r);
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_publish_without_version(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // publish heylib, a component without a dependency
    let component_dir = clone_component_dir("heylib", &state);
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    let mut build_opts = build::options(Some(&state.tempdir.path()), &env_name, &manifest).expect("build options");
    build_opts.version = None;
    build_opts.release = true;

    let r = build::build_with_options(&component_dir, &manifest, &env_name, &state.tempdir.path(), &build_opts);
    assert!(r.is_ok(), "built heylib without version: {:?}", r);

    let r = publish::publish_release(&component_dir, &state.backend, &state.tempdir.path());
    assert!(r.is_err(), "can't publish without version: {:?}", r);
}
