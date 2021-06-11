use crate::common::*;
use parameterized_macro::parameterized;

#[parameterized(env_name = {"default", "alpine"})]
fn test_build_and_stash(env_name: &str) {
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

    let r = stash::stash(&component_dir, &state.backend, "blah");
    assert!(r.is_ok(), "stashed heylib=blah artifact")
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_build_stashed_self_with_simple_verify(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");

    let mut build_opts = build::options(Some(&state.tempdir.path()), &env_name, &manifest).expect("build options");
    build_opts.name = Some("heylib".to_string());
    build_opts.version = None;

    // Initial build to generate a stashed component
    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");
    let r = build::build_with_options(&component_dir, &manifest, &env_name, &state.tempdir.path(), &build_opts);
    assert!(r.is_ok(), "built heylib for stashing");
    let r = stash::stash(&component_dir, &state.backend, "blah");
    assert!(r.is_ok(), "stashed heylib=blah artifact");

    // Fun fact: a component can depend on itself in INPUT.
    // This may not always make sense, but can be a way to compose features.
    let r = update::update(&component_dir, &env_name, &state.backend, vec!["heylib=blah"]);
    assert!(r.is_ok(), "updated heylib=blah from stash");

    // Build with the stashed dependency in INPUT
    // This will fail, since stashed dependencies are not available to anyone besides
    // yourself locally. A stashed dependency is never published.
    let r = build::build_with_options(&component_dir, &manifest, &env_name, &state.tempdir.path(), &build_opts);
    match r {
        Err(lal::CliError::NonGlobalDependencies(nonglobal)) => {
            assert_eq!(nonglobal, "heylib");
        }
        _ => {
            println!("Actual result was {:?}", r);
            assert!(false);
        }
    };

    // The build can succeed if using the `simple_verify` algorithm.
    build_opts.force = false;
    build_opts.simple_verify = true;
    let r = build::build_with_options(&component_dir, &manifest, &env_name, &state.tempdir.path(), &build_opts);
    assert!(
        r.is_ok(),
        "built heylib using stashed dependencies (simple_verify = true)"
    );
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_build_stashed_self_with_force(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);
    let manifest = lal::Manifest::read(&component_dir).expect("read manifest");

    let mut build_opts = build::options(Some(&state.tempdir.path()), &env_name, &manifest).expect("build options");
    build_opts.name = Some("heylib".into());
    build_opts.version = None;

    // Initial build to generate a stashed component
    let r = fetch::fetch_input(&component_dir, &env_name, &state.backend);
    assert!(r.is_ok(), "installed heylib dependencies");
    let r = build::build_with_options(&component_dir, &manifest, &env_name, &state.tempdir.path(), &build_opts);
    assert!(r.is_ok(), "built heylib for stashing");
    let r = stash::stash(&component_dir, &state.backend, "blah");
    assert!(r.is_ok(), "stashed heylib=blah artifact");

    // Fun fact: a component can depend on itself in INPUT.
    // This may not always make sense, but can be a way to compose features.
    let r = update::update(&component_dir, &env_name, &state.backend, vec!["heylib=blah"]);
    assert!(r.is_ok(), "updated heylib=blah from stash");

    // Build with the stashed dependency in INPUT
    // The build can succeed if using `force`.
    build_opts.force = true;
    build_opts.simple_verify = false;
    let r = build::build_with_options(&component_dir, &manifest, &env_name, &state.tempdir.path(), &build_opts);
    assert!(
        r.is_ok(),
        "built heylib using stashed dependencies (force = true)"
    );
}
