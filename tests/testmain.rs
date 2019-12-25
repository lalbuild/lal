extern crate lal;

extern crate fs_extra;
#[macro_use] extern crate log;
extern crate loggerv;
extern crate parameterized_macro;
extern crate tempdir;
extern crate walkdir;

mod cases;
mod common;
use common::*;

use std::path::Path;

use parameterized_macro::parameterized;

use lal::*;

#[parameterized(env_name = {"default", "alpine"})]
fn test_propagations(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // verify propagations by building prop-leaf -> prop-mid-X -> prop-base
    let component_dir = clone_component_dir("prop-leaf", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish prop-leaf");

    let component_dir = clone_component_dir("prop-mid-1", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish prop-mid-1");

    let component_dir = clone_component_dir("prop-mid-2", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish prop-mid-2");

    let component_dir = clone_component_dir("prop-base", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish prop-base");

    check_propagation(&component_dir, "prop-leaf");
    info!("ok check_propagation prop-leaf -> prop-base");
}

fn fetch_release_build_and_publish<T: CachedBackend + Backend>(
    component_dir: &Path,
    env_name: &str,
    backend: &T,
    home: &Path,
) {
    let cfg = Config::read(Some(&home)).unwrap();
    let mf = Manifest::read(&component_dir).unwrap();
    let environment = cfg.get_environment(env_name.into()).unwrap();

    let rcore = lal::fetch(&component_dir, &mf, backend, true, env_name);
    assert!(rcore.is_ok(), "install core succeeded");

    // we'll try with various build options further down with various deps
    let bopts = BuildOptions {
        name: None,
        configuration: Some("release".into()),
        environment: environment,
        release: true,
        version: Some("1".into()), // want to publish version 1 for later
        sha: None,
        force: false,
        simple_verify: false,
    };
    let modes = ShellModes::default();
    lal::build(&component_dir, &cfg, &mf, &bopts, env_name.into(), modes.clone())
        .expect("could build in release");

    let rp = lal::publish(Some(&home), &component_dir, &mf.name, backend);
    assert!(rp.is_ok(), "could publish");
}

fn check_propagation(component_dir: &Path, leaf: &str) {
    let mf = Manifest::read(&component_dir).unwrap();

    let lf = Lockfile::default()
        .set_name(&mf.name)
        .populate_from_input(&component_dir)
        .unwrap();
    if let Ok(res) = lal::propagate::compute(&lf, leaf) {
        assert_eq!(res.stages.len(), 2);
        // first stage
        assert_eq!(res.stages[0].updates.len(), 2); // must update both mid points
        assert_eq!(res.stages[0].updates[0].dependencies, vec!["prop-leaf"]);
        assert_eq!(res.stages[0].updates[1].dependencies, vec!["prop-leaf"]);
        assert_eq!(res.stages[0].updates[0].repo, "prop-mid-1");
        assert_eq!(res.stages[0].updates[1].repo, "prop-mid-2");
        // second stage
        assert_eq!(res.stages[1].updates.len(), 1); // must update base
        assert_eq!(res.stages[1].updates[0].dependencies, vec![
            "prop-mid-1",
            "prop-mid-2"
        ]);
        assert_eq!(res.stages[1].updates[0].repo, "prop-base");
    } else {
        assert!(false, "could propagate leaf to {}", mf.name);
    }

    let rpj = lal::propagate::print(&component_dir, &mf, leaf, true);
    assert!(rpj.is_ok(), "could print propagate json to stdout");
    let rp = lal::propagate::print(&component_dir, &mf, leaf, false);
    assert!(rp.is_ok(), "could print propagate to stdout");

    // print tree for extra coverage of bigger trees
    let rs = lal::status(&component_dir, &mf, true, true, true);
    assert!(rs.is_ok(), "could print status of propagation root");
}

#[cfg(feature = "upgrade")]
fn upgrade_does_not_fail() {
    let uc = lal::upgrade(true);
    assert!(uc.is_ok(), "could perform upgrade check");
    let upgraded = uc.unwrap();
    assert!(!upgraded, "we never have upgrades in the tip source tree");
}
