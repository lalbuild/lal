use crate::common::*;
use parameterized_macro::parameterized;
use std::process::Command;

#[parameterized(env_name = {"default", "alpine"})]
fn test_build(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies: {:?}", r);

    let r = build::build_for_release(&component_dir, &env_name, &state.tempdir.path(), "1");
    assert!(r.is_ok(), "built heylib release: {:?}", r);
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_build_with_force(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    // Force build the component
    let mut build_opts = build::options(Some(&state.tempdir.path()), &env_name, &manifest).expect("build options");
    build_opts.force = true;

    let r = build::build_with_options(&component_dir, &manifest, &env_name, &state.tempdir.path(), &build_opts);
    assert!(r.is_ok(), "built heylib with force");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_build_with_force_in_wrong_environment(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    // Force build the component
    let mut build_opts = build::options(Some(&state.tempdir.path()), &env_name, &manifest).expect("build options");
    build_opts.force = true;

    let r = build::build_with_options(&component_dir, &manifest, "nonexistant", &state.tempdir.path(), &build_opts);
    assert!(r.is_ok(), "built heylib with force in nonexistant environment");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_build_with_printonly(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");

    // Default build options
    let build_opts = build::options(Some(&state.tempdir.path()), &env_name, &manifest).expect("build options");

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

#[parameterized(env_name = {"default", "alpine"})]
fn test_build_with_release_create_tar_compatible_output(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies: {:?}", r);

    let r = build::build_for_release(&component_dir, &env_name, &state.tempdir.path(), "1");
    assert!(r.is_ok(), "built heylib release: {:?}", r);

    // Check if tar can extract the artifact archive
    let artifact = state.tempdir.path().join("heylib/ARTIFACT/heylib.tar.gz");
    let extracted = state.tempdir.path().join("EXTRACTED");
    std::fs::create_dir_all(&extracted).unwrap();

    let args: Vec<&str> = vec!["xvf", &artifact.to_str().unwrap(), "-C", "EXTRACTED"];

    // Spawn `tar` in a subprocess to test if it can read the artifact
    let r = Command::new("tar")
        .args(&args)
        .current_dir(&state.tempdir.path())
        .output();
    assert!(r.is_ok(), "Extracted heylib release archive: {:?}", r);

    // Report the test failure if tar could not extract the files
    let output = r.unwrap();
    let status = output.status;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        status.success(),
        "Subprocess failed: {:?}\n{}\n{}",
        status,
        stdout,
        stderr
    );

    // Check for the extracted file content
    for file in ["hey.h", "libhey.a", "lockfile.json"].iter() {
        let metadata = std::fs::metadata(&extracted.join(file));
        let metadata = metadata.expect(file);
        assert!(metadata.is_file());
    }
}
