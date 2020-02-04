use crate::common::*;
use parameterized_macro::parameterized;
use std::ffi::OsStr;

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
pub fn test_query(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1")
        .expect("published heylib=1 helloworld=1");

    let r = lal::query(&state.backend, env_name.to_str(), "hello", false);
    assert!(r.is_ok(), "could query for hello");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
pub fn test_query_last(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1")
        .expect("published heylib=1 helloworld=1");

    let r = lal::query(&state.backend, env_name.to_str(), "hello", true);
    assert!(r.is_ok(), "could query for hello");
}
