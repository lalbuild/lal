use crate::common::*;
use parameterized_macro::parameterized;
use std::path::PathBuf;

fn assert_missing_lockfile(path: &PathBuf, name: &str) {
    match lal::Lockfile::from_path(path, &name) {
        Err(lal::CliError::MissingLockfile(_)) => Ok(()),
        Ok(_) => Err("lockfile exists".to_string()),
        Err(ref e) => Err(format!("{}", e)),
    }
    .expect("dependency not in INPUT");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_status_without_deps(env_name: &str) {
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

    let r = status::status(&component_dir);
    assert!(r.is_ok(), "checked heylib dependency status");

    let r = status::full_status(&component_dir);
    assert!(r.is_ok(), "checked full heylib dependency status");

    let r = status::full_descriptive_status(&component_dir);
    assert!(r.is_ok(), "checked fully described heylib dependencies");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_status_with_deps(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // heylib component is a dependency
    publish_component(&state, &env_name, "heylib", "1").expect("published heylib=1");

    // helloworld depends on heylib
    let component_dir = clone_component_dir("helloworld", &state);

    // INPUT dependencies are not ready yet
    let r = status::status(&component_dir);
    assert!(r.is_ok(), "helloworld deps not fetched");
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed helloworld dependencies");
    let lockfile = lal::Lockfile::from_path(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib")
        .expect("read heylib=1 lockfile");
    assert_eq!(lockfile.name, "heylib".to_string());
    assert_eq!(lockfile.version, "1".to_string());

    let r = status::status(&component_dir);
    assert!(r.is_ok(), "checked helloworld dependency status");

    let r = status::full_status(&component_dir);
    assert!(r.is_ok(), "checked full helloworld dependency status");

    let r = status::full_descriptive_status(&component_dir);
    assert!(r.is_ok(), "checked fully described helloworld dependencies");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_status_with_stashed_dependency(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Initial build to generate a published and a stashed component
    publish_component(&state, &env_name, "heylib", "1").expect("published heylib=1");
    stash_component(&state, &env_name, "heylib", "blah").expect("published heylib=blah");

    // helloworld depends on heylib
    let component_dir = clone_component_dir("helloworld", &state);

    // Dependencies are not yet in INPUT
    let r = status::status(&component_dir);
    assert!(r.is_ok(), "helloworld has no inputs");
    assert_missing_lockfile(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib");

    // Fetch the published dependency
    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "fetched published dependencies for helloworld");

    let r = status::status(&component_dir);
    assert!(r.is_ok(), "helloworld has published dependencies");
    let lockfile = lal::Lockfile::from_path(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib")
        .expect("read heylib=1 lockfile");
    assert_eq!(lockfile.name, "heylib".to_string());
    assert_eq!(lockfile.version, "1".to_string());

    // Switch to the stashed dependencies, since these are local (a stashed component is never
    // published, and cannot be consumed by other builds) status checks fail.
    let r = update::update(&component_dir, &env_name, &state.backend, vec!["heylib=blah"]);
    assert!(r.is_ok(), "updated heylib=blah from stash");

    let r = status::status(&component_dir);
    assert!(r.is_ok(), "helloworld has unpublished (stashed) dependencies");
    let lockfile = lal::Lockfile::from_path(&component_dir.join("INPUT/heylib/lockfile.json"), "heylib")
        .expect("read heylib=blah lockfile");
    assert_eq!(lockfile.name, "heylib".to_string());
    assert_eq!(lockfile.version, "blah".to_string());
}
