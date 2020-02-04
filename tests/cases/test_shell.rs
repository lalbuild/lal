use crate::common::*;
use parameterized_macro::parameterized;
use std::ffi::OsStr;

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
pub fn test_heylib_echo(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = shell::run(&env_name, &state.tempdir.path(), &component_dir, vec![
        "echo",
        "# echo from docker",
    ]);
    assert!(r.is_ok(), "shell echoed");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
pub fn test_shell_permissions(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = shell::run(&env_name, &state.tempdir.path(), &component_dir, vec![
        "touch",
        "README.md",
    ]);
    assert!(r.is_ok(), "could touch files in container");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
pub fn test_run_scripts(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    let r = shell::run_script(
        &env_name,
        &state.tempdir.path(),
        &component_dir,
        "subroutine",
        vec!["Hello", "World"],
    );
    assert!(r.is_ok(), "could run `subroutine` script");
}
