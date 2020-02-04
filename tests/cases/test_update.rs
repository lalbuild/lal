use crate::common::*;
use parameterized_macro::parameterized;
use std::{ffi::OsStr, path::{Path, PathBuf}};

fn assert_missing_lockfile(path: &PathBuf, name: &str) {
    match lal::Lockfile::from_path(path, &name) {
        Err(lal::CliError::MissingLockfile(_)) => Ok(()),
        Ok(_) => Err("lockfile exists".to_string()),
        Err(ref e) => Err(format!("{}", e)),
    }
    .expect("dependency not in INPUT");
}

fn assert_lockfile(lockfile: &PathBuf, name: &str, version: u32) {
    let lockfile = lal::Lockfile::from_path(&lockfile, name).expect("read lockfile");
    assert_eq!(lockfile.name, name.to_string());
    assert_eq!(lockfile.version, version.to_string());
}

fn assert_manifest(component_dir: &Path, name: &str, version: u32) {
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");
    assert_eq!(
        manifest.dependencies.get_key_value(name),
        Some((&name.to_string(), &version))
    );
}

fn assert_manifest_dev(component_dir: &Path, name: &str, version: u32) {
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");
    assert_eq!(
        manifest.devDependencies.get_key_value(name),
        Some((&name.to_string(), &version))
    );
}

fn assert_missing_manifest(component_dir: &Path, name: &str) {
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");
    assert!(!manifest.dependencies.contains_key(name));
}

fn assert_missing_manifest_dev(component_dir: &Path, name: &str) {
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");
    assert!(!manifest.devDependencies.contains_key(name));
}

// -- Update without saving

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_no_save(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    // Initial manifest is at version 1
    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    // lal update heylib=1
    let r = update::update(&component_dir, &env_name, &state.backend, vec!["heylib=1"]);
    assert!(r.is_ok(), "updated heylib=1");

    assert_manifest(&component_dir, "heylib", 1);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 1);

    // lal update heylib=2
    let r = update::update(&component_dir, &env_name, &state.backend, vec!["heylib=2"]);
    assert!(r.is_ok(), "updated heylib=2");

    // Manifest hasn't been updated, but INPUT has specified version
    assert_manifest(&component_dir, "heylib", 1);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_to_latest_no_save(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    // Initial manifest is at version 1
    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    // lal update heylib=1
    let r = update::update(&component_dir, &env_name, &state.backend, vec!["heylib=1"]);
    assert!(r.is_ok(), "updated heylib=1");
    assert_manifest(&component_dir, "heylib", 1);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 1);

    // lal update heylib, should pick up the latest version
    let r = update::update(&component_dir, &env_name, &state.backend, vec!["heylib"]);
    assert!(r.is_ok(), "updated heylib");

    // latest version is 2, without saving the manifest
    assert_manifest(&component_dir, "heylib", 1);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_all_to_latest_no_save(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    // Initial manifest is at version 1
    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    // lal update heylib=1
    let r = update::update(&component_dir, &env_name, &state.backend, vec!["heylib=1"]);
    assert!(r.is_ok(), "updated heylib=1");
    assert_manifest(&component_dir, "heylib", 1);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 1);

    // lal update-all, should pick up the latest version
    let r = update::update_all(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "updated all helloworld INPUTs");

    // latest version is 2, manifest unchanged
    assert_manifest(&component_dir, "heylib", 1);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);
}

// -- Update and save to the manifest

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_with_save(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    let save = true;
    let savedev = false;

    // lal update --save heylib=1
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["heylib=1"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib=1");
    assert_manifest(&component_dir, "heylib", 1);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 1);

    // lal update --save heylib=2
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["heylib=2"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib=2");
    assert_manifest(&component_dir, "heylib", 2);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_to_latest_with_save(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    let save = true;
    let savedev = false;

    // lal update --save heylib=1
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["heylib=1"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib=1");
    assert_manifest(&component_dir, "heylib", 1);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 1);

    // lal update --save heylib, should pick up the latest version
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["heylib"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib");

    // latest version is 2
    assert_manifest(&component_dir, "heylib", 2);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_all_to_latest_with_save(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    let save = true;
    let savedev = false;

    // lal update --save heylib=1
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["heylib=1"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib=1");
    assert_manifest(&component_dir, "heylib", 1);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 1);

    // lal update-all --save, should pick up the latest version
    let r = update::update_all_with_save(&component_dir, &env_name, &state.backend, save, savedev);
    assert!(r.is_ok(), "updated all helloworld INPUTs");

    // latest version is 2
    assert_manifest(&component_dir, "heylib", 2);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);
}

// -- Update and savedev to the manifest

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_with_savedev(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    let r = publish_component_versions(&state, &env_name, "prop-leaf", vec!["1", "2"]);
    assert!(r.is_ok(), "published prop-leaf=1 and prop-leaf=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    let save = false;
    let savedev = true;

    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_manifest_dev(&component_dir, "heylib");

    assert_missing_manifest(&component_dir, "prop-leaf");
    assert_missing_manifest_dev(&component_dir, "prop-leaf");

    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");
    assert_missing_lockfile(&component_dir.join("INPUT/prop-leaf/lockfile.json"), "prop-leaf");

    // lal update --dev prop-leaf=1
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["prop-leaf=1"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib=1");
    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_manifest_dev(&component_dir, "heylib");

    assert_manifest_dev(&component_dir, "prop-leaf", 1);
    assert_missing_manifest(&component_dir, "prop-leaf");

    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        1,
    );

    // lal update --dev heylib=2 prop-leaf=2
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["heylib=2", "prop-leaf=2"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib=2 prop-leaf=2");

    // XXX: Bug? Why isn't the core dependency updated in the manifest?
    //      Does it make sense for the dev dependency to be at a different version?
    //      Does it make sense for a component to be in both core and dev dependencies?
    assert_manifest(&component_dir, "heylib", 1);
    assert_manifest_dev(&component_dir, "heylib", 2);

    assert_manifest_dev(&component_dir, "prop-leaf", 2);
    assert_missing_manifest(&component_dir, "prop-leaf");

    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        2,
    );
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_with_save_savedev(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    let r = publish_component_versions(&state, &env_name, "prop-leaf", vec!["1", "2"]);
    assert!(r.is_ok(), "published prop-leaf=1 and prop-leaf=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    let save = true;
    let savedev = true;

    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_manifest_dev(&component_dir, "heylib");

    assert_missing_manifest(&component_dir, "prop-leaf");
    assert_missing_manifest_dev(&component_dir, "prop-leaf");

    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");
    assert_missing_lockfile(&component_dir.join("INPUT/prop-leaf/lockfile.json"), "prop-leaf");

    // lal update --save --dev prop-leaf=1
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["prop-leaf=1"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib=1");
    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_manifest_dev(&component_dir, "heylib");

    // XXX: Should this be a dev dependency?
    assert_manifest(&component_dir, "prop-leaf", 1);
    assert_missing_manifest_dev(&component_dir, "prop-leaf");

    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        1,
    );

    // lal update --save --dev heylib=2 prop-leaf=2
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["heylib=2", "prop-leaf=2"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib=2 prop-leaf=2");

    // XXX: Bug? Why isn't the core dependency updated in the manifest?
    //      Does it make sense for the dev dependency to be at a different version?
    //      Does it make sense for a component to be in both core and dev dependencies?
    assert_manifest(&component_dir, "heylib", 2);
    assert_missing_manifest_dev(&component_dir, "heylib");

    // XXX: What's the point of the --dev flag here?
    assert_manifest(&component_dir, "prop-leaf", 2);
    assert_missing_manifest_dev(&component_dir, "prop-leaf");

    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        2,
    );
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_to_latest_with_savedev(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    let r = publish_component_versions(&state, &env_name, "prop-leaf", vec!["1", "2"]);
    assert!(r.is_ok(), "published prop-leaf=1 and prop-leaf=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    let save = false;
    let savedev = true;

    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    assert_missing_manifest_dev(&component_dir, "prop-leaf");
    assert_missing_lockfile(&component_dir.join("INPUT/prop-leaf/lockfile.json"), "prop-leaf");

    // lal update --dev prop-leaf
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["prop-leaf"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib=1");

    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    assert_manifest_dev(&component_dir, "prop-leaf", 2);
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        2,
    );

    // lal update --dev heylib, should pick up the latest version
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["heylib"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib");

    // Remains as a core dependency
    // XXX: Bug that the core dependency isn't bumped?
    //      But we have a new entry in devDependencies?
    assert_manifest(&component_dir, "heylib", 1);
    assert_manifest_dev(&component_dir, "heylib", 2);
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);

    assert_missing_manifest(&component_dir, "prop-leaf");
    assert_manifest_dev(&component_dir, "prop-leaf", 2);
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        2,
    );
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_to_latest_with_save_savedev(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    let r = publish_component_versions(&state, &env_name, "prop-leaf", vec!["1", "2"]);
    assert!(r.is_ok(), "published prop-leaf=1 and prop-leaf=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    let save = true;
    let savedev = true;

    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    assert_missing_manifest_dev(&component_dir, "prop-leaf");
    assert_missing_lockfile(&component_dir.join("INPUT/prop-leaf/lockfile.json"), "prop-leaf");

    // lal update --save --dev prop-leaf
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["prop-leaf"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib=1");

    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_manifest_dev(&component_dir, "heylib");
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    // XXX: What's the point of the --dev flag, if devDependencies are not updated?
    assert_manifest(&component_dir, "prop-leaf", 2);
    assert_missing_manifest_dev(&component_dir, "prop-leaf");
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        2,
    );

    // lal update --save --dev heylib prop-leaf, should pick up the latest versions
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["heylib", "prop-leaf"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated heylib");

    // XXX: Just documenting current behaviour here
    //      Not sure if this is completely right, perhaps needs a section in the spec.
    //      Please change these checks if behaviour is later modified.
    assert_manifest(&component_dir, "heylib", 2);
    assert_missing_manifest_dev(&component_dir, "heylib");
    assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);

    assert_manifest(&component_dir, "prop-leaf", 2);
    assert_missing_manifest_dev(&component_dir, "prop-leaf");
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        2,
    );
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_all_to_latest_with_savedev(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    let r = publish_component_versions(&state, &env_name, "prop-leaf", vec!["1", "2"]);
    assert!(r.is_ok(), "published prop-leaf=1 and prop-leaf=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    let save = false;
    let savedev = true;

    // lal update --dev prop-leaf=1
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["prop-leaf=1"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated prop-leaf=1");

    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_manifest_dev(&component_dir, "heylib");
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    assert_manifest_dev(&component_dir, "prop-leaf", 1);
    assert_missing_manifest(&component_dir, "prop-leaf");
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        1,
    );

    // lal update-all --dev, should pick up the latest versions of core and dev dependencies
    let r = update::update_all_with_save(&component_dir, &env_name, &state.backend, save, savedev);
    assert!(r.is_ok(), "updated all helloworld INPUTs");

    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_manifest_dev(&component_dir, "heylib");

    // XXX: Should core dependencies be updated too?
    // assert_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib", 2);
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    assert_manifest_dev(&component_dir, "prop-leaf", 1);
    assert_missing_manifest(&component_dir, "prop-leaf");
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        2,
    );
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_update_all_to_latest_with_save_savedev(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_component_versions(&state, &env_name, "heylib", vec!["1", "2"]);
    assert!(r.is_ok(), "published heylib=1 and heylib=2");

    let r = publish_component_versions(&state, &env_name, "prop-leaf", vec!["1", "2"]);
    assert!(r.is_ok(), "published prop-leaf=1 and prop-leaf=2");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    let save = true;
    let savedev = true;

    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    assert_missing_manifest_dev(&component_dir, "prop-leaf");
    assert_missing_lockfile(&component_dir.join("INPUT/prop-leaf/lockfile.json"), "prop-leaf");

    // lal update --save --dev prop-leaf=1
    let r = update::update_with_save(
        &component_dir,
        &env_name,
        &state.backend,
        vec!["prop-leaf=1"],
        save,
        savedev,
    );
    assert!(r.is_ok(), "updated prop-leaf=1");

    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_manifest_dev(&component_dir, "heylib");
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    // XXX: This feels like it did completely the wrong thing. Thoughts?
    assert_manifest(&component_dir, "prop-leaf", 1);
    assert_missing_manifest_dev(&component_dir, "prop-leaf");
    // should be
    // assert_manifest_dev(&component_dir, "prop-leaf", 1);
    // assert_missing_manifest(&component_dir, "prop-leaf");

    // At least the lockfile is right
    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        1,
    );

    // lal update-all --save --dev, should pick up the latest versions of core and dev dependencies
    let r = update::update_all_with_save(&component_dir, &env_name, &state.backend, save, savedev);
    assert!(r.is_ok(), "updated all helloworld INPUTs");

    // BUG: should be at version 2
    assert_manifest(&component_dir, "heylib", 1);
    assert_missing_manifest_dev(&component_dir, "heylib");
    // BUG: Where's the dependency gone?
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    // XXX: ¯\_(ツ)_/¯, why even have a --dev flag?
    // BUG: should be at version 2
    assert_manifest(&component_dir, "prop-leaf", 1);
    assert_missing_manifest_dev(&component_dir, "prop-leaf");

    assert_lockfile(
        &component_dir.join("INPUT/prop-leaf/lockfile.json"),
        "prop-leaf",
        1,
    );
}
