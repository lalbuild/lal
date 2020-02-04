use crate::common::*;
use parameterized_macro::parameterized;
use std::{ffi::OsStr, path::PathBuf};

fn publish_components(state: &TestState, env_name: &OsStr) -> lal::LalResult<PathBuf> {
    // verify propagations by building prop-leaf -> prop-mid-X -> prop-base
    publish_component(&state, &env_name, "prop-leaf", "1")?;
    publish_component(&state, &env_name, "prop-mid-1", "1")?;
    publish_component(&state, &env_name, "prop-mid-2", "1")?;
    publish_component(&state, &env_name, "prop-base", "1")
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_propagate_compute(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = publish_components(&state, &env_name).expect("published components");

    // prop-leaf is a deep dependency, compute what else needs to be
    // updated, in order to update the current component.
    let update_sequence =
        propagate::compute(&component_dir, "prop-leaf").expect("computing propagation sequence");

    // Inspect the update sequence
    assert_eq!(update_sequence.stages.len(), 2);

    // first stage
    assert_eq!(update_sequence.stages[0].updates.len(), 2); // must update both mid points

    assert_eq!(update_sequence.stages[0].updates[0].repo, "prop-mid-1");
    assert_eq!(update_sequence.stages[0].updates[0].dependencies, vec![
        "prop-leaf"
    ]);

    assert_eq!(update_sequence.stages[0].updates[1].repo, "prop-mid-2");
    assert_eq!(update_sequence.stages[0].updates[1].dependencies, vec![
        "prop-leaf"
    ]);

    // second stage
    assert_eq!(update_sequence.stages[1].updates.len(), 1); // must update base

    assert_eq!(update_sequence.stages[1].updates[0].repo, "prop-base");
    assert_eq!(update_sequence.stages[1].updates[0].dependencies, vec![
        "prop-mid-1",
        "prop-mid-2"
    ]);

    // print tree for extra coverage of bigger trees
    let r = status::full_descriptive_status(&component_dir);
    assert!(r.is_ok(), "could print status of propagation root");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_propagate_print(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = publish_components(&state, &env_name).expect("published components");

    let r = propagate::print(&component_dir, "prop-leaf");
    assert!(r.is_ok(), "pretty printed propagation tree");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_propagate_print_json(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = publish_components(&state, &env_name).expect("published components");

    let r = propagate::print_json(&component_dir, "prop-leaf");
    assert!(r.is_ok(), "pretty printed propagation tree");
}
