use crate::common::*;
use parameterized_macro::parameterized;
use std::fs;

#[parameterized(env_name = {"default", "alpine"})]
fn test_change_envs(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component_dir = state.tempdir.path().join("new_component");
    fs::create_dir(&component_dir).expect("create new_component dir");

    let r = init::init(&component_dir, &env_name, &state.tempdir.path());
    assert!(r.is_ok(), "new component created: {:?}", r);

    // Read sticky options
    let sticky = lal::StickyOptions::read(&component_dir).expect("read sticky options");

    // No environment override set
    assert_eq!(sticky.env, None);

    // Change env (even works with unsupported envs)
    configure_test_environment(&state.tempdir.path(), "xenial");
    let r = envs::set_environment(&component_dir, &state.tempdir.path(), &sticky, "xenial");
    assert!(r.is_ok(), "environment set to xenial: {:?}", r);

    // Reread sticky options
    let sticky = lal::StickyOptions::read(&component_dir).expect("reread sticky options");

    assert_eq!(sticky.env, Some("xenial".to_string()));

    // Unset environment override
    let r = envs::clear_environment(&component_dir);
    assert!(r.is_ok(), "cleared the environment");

    // Read sticky options
    let sticky = lal::StickyOptions::read(&component_dir).expect("read sticky options");

    // Environment override cleared
    assert_eq!(sticky.env, None);
}
