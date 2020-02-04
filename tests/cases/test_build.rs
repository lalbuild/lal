use crate::common::*;
use parameterized_macro::parameterized;
use std::ffi::OsStr;

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_build(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    let r = build::build_for_release(&component_dir, &env_name, &state.tempdir.path(), "1");
    assert!(r.is_ok(), "built heylib release");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_build_with_force(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    // Force build the component
    let mut build_opts = build::options(Some(&state.tempdir.path()), &env_name).expect("build options");
    build_opts.force = true;

    let r = build::build_with_options(&component_dir, &env_name, &state.tempdir.path(), &build_opts);
    assert!(r.is_ok(), "built heylib with force");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_build_with_force_in_wrong_environment(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    // Force build the component
    let mut build_opts = build::options(Some(&state.tempdir.path()), &env_name).expect("build options");
    build_opts.force = true;

    let r = build::build_with_options(&component_dir, OsStr::new("nonexistant"), &state.tempdir.path(), &build_opts);
    assert!(r.is_ok(), "built heylib with force in nonexistant environment");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_build_with_printonly(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);
    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    // Default build options
    let build_opts = build::options(Some(&state.tempdir.path()), &env_name).expect("build options");

    // Print commands, don't execute
    let mut modes = lal::ShellModes::default();
    modes.printonly = true;

    // Only print commands for build
    let r = build::build_with_options_and_modes(
        &component_dir,
        &env_name,
        &state.tempdir.path(),
        &build_opts,
        modes,
    );
    assert!(r.is_ok(), "built heylib (printonly = true)");
}
