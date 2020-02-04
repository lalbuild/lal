use crate::common::*;
use parameterized_macro::parameterized;
use std::ffi::OsStr;

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_list_environments(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let r = publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1");
    assert!(r.is_ok(), "published heylib=1 helloworld=1");

    // TODO: Assert output
    let r = list::list_environments(&state.tempdir.path());
    assert!(r.is_ok(), "list environments");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_list_core_dependencies(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1")
        .expect("published heylib=1 helloworld=1");

    // TODO: Assert output
    let core = true;
    let r = list::list_dependencies(&component_dir, core);
    assert!(r.is_ok(), "list core dependencies");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_list_all_dependencies(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1")
        .expect("published heylib=1 helloworld=1");

    // TODO: Assert output
    let core = false;
    let r = list::list_dependencies(&component_dir, core);
    assert!(r.is_ok(), "list all dependencies");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_list_configurations(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1")
        .expect("published heylib=1 helloworld=1");

    let r = list::list_configurations(&component_dir);
    assert!(r.is_ok(), "list configurations");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
fn test_list_buildables(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1")
        .expect("published heylib=1 helloworld=1");

    let r = list::list_buildables(&component_dir);
    assert!(r.is_ok(), "list buildables");
}
