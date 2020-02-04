use crate::common::*;
use parameterized_macro::parameterized;
use std::ffi::OsStr;

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
pub fn test_clean_keep_1_day(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let config = lal::Config::read(Some(&state.tempdir.path())).expect("read config");

    publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1")
        .expect("published heylib=1 helloworld=1");

    let keep_days = 1;
    let r = lal::clean(&config.cache, keep_days);
    assert!(r.is_ok(), "keeping 1 day of cached artifacts");

    // Fresh components are kept
    let mut dirs = walkdir::WalkDir::new(&config.cache)
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir());

    let first = dirs.next();
    assert!(first.is_some(), "some artifacts cached since last time");
}

#[parameterized(env_name = {OsStr::new("default"), OsStr::new("alpine")})]
pub fn test_clean_keep_nothing(env_name: &OsStr) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let config = lal::Config::read(Some(&state.tempdir.path())).expect("read config");

    publish_components(&state, &env_name, vec!["heylib", "helloworld"], "1")
        .expect("published heylib=1 helloworld=1");

    let keep_days = 0;
    let r = lal::clean(&config.cache, keep_days);
    assert!(r.is_ok(), "keeping 0 days of cached artifacts");

    // Fresh components are kept
    let mut dirs = walkdir::WalkDir::new(&config.cache)
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir());

    let first = dirs.next();
    assert!(first.is_none(), "no artifacts left in cache");
}
