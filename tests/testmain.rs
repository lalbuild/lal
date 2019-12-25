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

use std::{fs, fs::File, io::prelude::*, path::Path, process::Command};

use parameterized_macro::parameterized;
use walkdir::WalkDir;

use lal::*;

#[parameterized(env_name = {"default", "alpine"})]
fn test_heylib_echo(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    shell_echo(&env_name, &state.tempdir.path(), &component_dir);
    info!("ok shell_echo");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_shell_permissions(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    shell_permissions(&env_name, &state.tempdir.path(), &component_dir);
    info!("ok shell_permissions");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_run_scripts(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    run_scripts(&env_name, &state.tempdir.path(), &component_dir);
    info!("ok run_scripts");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_build_and_stash_update_self(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);
    build_and_stash_update_self(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok build_and_stash_update_self");

    list_everything(&state.tempdir.path(), &component_dir);
    info!("ok list_everything");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_status_on_experimental(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // Test basic build functionality with heylib component
    let component_dir = clone_component_dir("heylib", &state);

    build_and_stash_update_self(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok build_and_stash_update_self");

    status_on_experimentals(&component_dir);
    info!("ok status_on_experimentals");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_verify_checks(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // "helloworld" depends on "heylib"
    let component_dir = clone_component_dir("heylib", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish heylib");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);

    verify_checks(&component_dir, &env_name, &state.backend);
    info!("ok verify_checks");
}


#[parameterized(env_name = {"default", "alpine"})]
fn test_release_build_and_publish(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // "helloworld" depends on "heylib"
    let component_dir = clone_component_dir("heylib", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish heylib");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish helloworld");

    list_everything(&state.tempdir.path(), &component_dir);
    info!("ok list_everything");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_remove_dependencies(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // "helloworld" has 1 dependency
    let component_dir = clone_component_dir("helloworld", &state);

    remove_dependencies(&component_dir);
    info!("ok remove_dependencies");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_export_checks(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // "helloworld" depends on "heylib"
    let component_dir = clone_component_dir("heylib", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish heylib");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish helloworld");

    // back to tmpdir to test export and clean
    export_check(&env_name, &state.backend, &component_dir);
    info!("ok export_check");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_query_check(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // "helloworld" depends on "heylib"
    let component_dir = clone_component_dir("heylib", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish heylib");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish helloworld");

    query_check(&env_name, &state.backend);
    info!("ok query_check");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_clean_check(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    // "helloworld" depends on "heylib"
    let component_dir = clone_component_dir("heylib", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish heylib");

    // switch to "helloworld" component
    let component_dir = clone_component_dir("helloworld", &state);
    fetch_release_build_and_publish(&component_dir, &env_name, &state.backend, &state.tempdir.path());
    info!("ok fetch_release_build_and_publish helloworld");

    clean_check(&state.tempdir.path());
    info!("ok clean_check");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_init_force(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component = state.tempdir.path().join("new_component");
    fs::create_dir(&component).unwrap();

    // test out some functionality regarding creating of new components
    init_force(&env_name, &state.tempdir.path(), &component);
    info!("ok init_force");

    has_config_and_manifest(&state.tempdir.path(), &component);
    info!("ok has_config_and_manifest");

    list_everything(&state.tempdir.path(), &component);
    info!("ok list_everything");
}

#[parameterized(env_name = {"default", "alpine"})]
fn test_change_envs(env_name: &str) {
    let state = setup();
    if !cfg!(feature = "docker") && env_name == "alpine" {
        return;
    }

    let component = state.tempdir.path().join("new_component");
    fs::create_dir(&component).unwrap();

    init_force(&env_name, &state.tempdir.path(), &component);
    info!("ok init_force");

    change_envs(&state.tempdir.path(), &component);
    info!("ok change_envs");
}


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

fn remove_dependencies(component_dir: &Path) {
    let mf = Manifest::read(&component_dir).unwrap();
    let xs = mf.dependencies.keys().cloned().collect::<Vec<_>>();
    assert_eq!(xs.len(), 1);

    let r = lal::remove(&component_dir, &mf, xs.clone(), false, false);
    assert!(r.is_ok(), "could lal rm all dependencies");

    let rs = lal::remove(&component_dir, &mf, xs, true, false);
    assert!(rs.is_ok(), "could lal rm all dependencies and save");

    // should be no dependencies now
    let mf2 = Manifest::read(&component_dir).unwrap();
    let xs2 = mf2.dependencies.keys().cloned().collect::<Vec<_>>();
    assert_eq!(xs2.len(), 0);
}

fn change_envs(home: &Path, component_dir: &Path) {
    let cfg = Config::read(Some(&home)).unwrap();
    let mf = Manifest::read(&component_dir).unwrap();

    // no sticky flags set yet
    let sticky_none = StickyOptions::read(&component_dir).unwrap();
    assert_eq!(sticky_none.env, None);

    // update the container associated with the default env
    // (on CI we've already done this at test start => cheap)
    let environment = cfg.get_environment(mf.environment.clone()).unwrap();
    let ru = lal::env::update(&component_dir, &environment, &mf.environment);
    assert!(ru.is_ok(), "env update succeeded");

    let rc = lal::env::set(&component_dir, &sticky_none, &cfg, "xenial");
    assert!(rc.is_ok(), "env set xenial succeeded");

    // we changed the sticky option with that
    let sticky_set = StickyOptions::read(&component_dir).unwrap();
    assert_eq!(sticky_set.env, Some("xenial".into()));

    let rc = lal::env::clear(&component_dir);
    assert!(rc.is_ok(), "env clear succeeded");

    // we cleared the stickies with that
    let sticky_clear = StickyOptions::read(&component_dir).unwrap();
    assert_eq!(sticky_clear.env, None);
}

fn list_everything(home: &Path, component_dir: &Path) {
    let cfg = Config::read(Some(&home)).unwrap();
    let mf = Manifest::read(&component_dir).unwrap();

    let re = lal::list::environments(&cfg);
    assert!(re.is_ok(), "list envs succeeded");

    let rdc = lal::list::dependencies(&mf, true);
    assert!(rdc.is_ok(), "list deps --core succeeded");
    let rd = lal::list::dependencies(&mf, false);
    assert!(rd.is_ok(), "list deps succeeded");

    let rc = lal::list::configurations(&mf.name, &mf);
    assert!(rc.is_ok(), "list configurations succeeded");

    let rb = lal::list::buildables(&mf);
    assert!(rb.is_ok(), "list buildables succeeded");
}

// Create manifest in a weird directory
fn init_force(env_name: &str, home: &Path, component_dir: &Path) {
    let cfg = Config::read(Some(&home)).expect("read config");

    let m1 = Manifest::read(&component_dir);
    assert!(m1.is_err(), "no manifest at this point");

    // Creates a manifest in the testtmp directory
    let m2 = lal::init(&cfg, false, &component_dir, env_name);
    assert!(m2.is_ok(), "could init without force param");

    let m3 = lal::init(&cfg, true, &component_dir, env_name);
    assert!(m3.is_ok(), "could re-init with force param");

    let m4 = lal::init(&cfg, false, &component_dir, env_name);
    assert!(m4.is_err(), "could not re-init without force ");

    let m5 = lal::init(&cfg, true, &component_dir, "blah");
    assert!(m5.is_err(), "could not init without valid environment");
}

// Tests need to be run in a directory with a manifest
// and ~/.lal + config must exist
fn has_config_and_manifest(home: &Path, component_dir: &Path) {
    assert!(home.is_dir(), "have laldir");

    let cfg = Config::read(Some(&home));
    assert!(cfg.is_ok(), "could read config");

    let manifest = Manifest::read(&component_dir);
    assert!(manifest.is_ok(), "could read manifest");

    // There is no INPUT yet, but we have no dependencies, so this should work:
    let r = lal::verify(&component_dir, &manifest.unwrap(), "xenial".into(), false);
    assert!(r.is_ok(), "could verify after install");
}

// Shell tests
fn shell_echo(env_name: &str, home: &Path, component_dir: &Path) {
    let cfg = Config::read(Some(&home)).unwrap();
    let environment = cfg.get_environment(env_name.into()).unwrap();
    let modes = ShellModes::default();
    let r = lal::run(
        &cfg,
        &environment,
        vec!["echo".to_string(), "# echo from docker".to_string()],
        &DockerRunFlags::default(),
        &modes,
        &component_dir,
    );
    assert!(r.is_ok(), "shell echoed");
}

fn shell_permissions(env_name: &str, home: &Path, component_dir: &Path) {
    let cfg = Config::read(Some(&home)).unwrap();
    let environment = cfg.get_environment(env_name.into()).unwrap();
    let modes = ShellModes::default();
    let r = lal::run(
        &cfg,
        &environment,
        vec!["touch".to_string(), "README.md".to_string()],
        &DockerRunFlags::default(),
        &modes,
        &component_dir,
    );
    assert!(r.is_ok(), "could touch files in container");
}

fn build_and_stash_update_self<T: CachedBackend + Backend>(
    component_dir: &Path,
    env_name: &str,
    backend: &T,
    home: &Path,
) {
    let cfg = Config::read(Some(&home)).expect("Read config");
    let mf = Manifest::read(&component_dir).expect("Read manifest");
    let environment = cfg.get_environment(env_name.into()).expect("get environment");

    // we'll try with various build options further down with various deps
    let mut bopts = BuildOptions {
        name: Some("heylib".into()),
        configuration: Some("release".into()),
        environment: environment,
        release: true,
        version: None,
        sha: None,
        force: false,
        simple_verify: false,
    };
    let modes = ShellModes::default();
    // basic build works - all deps are global at right env
    let r = lal::build(&component_dir, &cfg, &mf, &bopts, env_name.into(), modes.clone());
    if let Err(e) = r {
        println!("error from build: {:?}", e);
        assert!(false, "could perform a '{}' build", env_name);
    }

    // lal stash blah
    let rs = lal::stash(&component_dir, backend, &mf, "blah");
    assert!(rs.is_ok(), "could stash lal build artifact");

    // lal update heylib=blah
    let ru = lal::update(
        &component_dir,
        &mf,
        backend,
        vec!["heylib=blah".to_string()],
        false,
        false,
        "garbage", // env not relevant for stash
    );
    assert!(ru.is_ok(), "could update heylib from stash");

    // basic build won't work now without simple verify
    let r1 = lal::build(&component_dir, &cfg, &mf, &bopts, env_name.into(), modes.clone());
    assert!(r1.is_err(), "could not verify a new '{}' build", env_name);
    if let Err(CliError::NonGlobalDependencies(nonglob)) = r1 {
        assert_eq!(nonglob, "heylib");
    } else {
        println!("actual r1 was {:?}", r1);
        assert!(false);
    }

    bopts.simple_verify = true;
    let r2 = lal::build(&component_dir, &cfg, &mf, &bopts, env_name.into(), modes.clone());
    assert!(r2.is_ok(), "can build with stashed deps with simple verify");

    // force will also work - even with stashed deps from wrong env
    let renv = lal::build(&component_dir, &cfg, &mf, &bopts, "xenial".into(), modes.clone());
    assert!(renv.is_err(), "cannot build with simple verify when wrong env");
    if let Err(CliError::EnvironmentMismatch(_, compenv)) = renv {
        assert_eq!(compenv, env_name); // expected complaints about xenial env
    } else {
        println!("actual renv was {:?}", renv);
        assert!(false);
    }

    // settings that reflect lal build -f
    bopts.simple_verify = false;
    bopts.force = true;
    let renv2 = lal::build(&component_dir, &cfg, &mf, &bopts, "xenial".into(), modes.clone());
    assert!(renv2.is_ok(), "could force build in different env");

    // additionally do a build with printonly
    let all_modes = ShellModes {
        printonly: true,
        x11_forwarding: true,
        host_networking: true,
        env_vars: vec![],
    };
    let printbuild = lal::build(&component_dir, &cfg, &mf, &bopts, "alpine".into(), all_modes);
    // TODO: verify output!
    assert!(printbuild.is_ok(), "saw docker run print with X11 mounts");
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

fn verify_checks<T: CachedBackend + Backend>(component_dir: &Path, env_name: &str, backend: &T) {
    let mf = Manifest::read(&component_dir).unwrap();

    let rcore = lal::fetch(&component_dir, &mf, backend, true, env_name);
    assert!(rcore.is_ok(), "install core succeeded");

    let r = lal::verify(&component_dir, &mf, env_name.into(), false);
    assert!(r.is_ok(), "could verify after install");

    let renv1 = lal::verify(&component_dir, &mf, "xenial".into(), false);
    assert!(renv1.is_err(), "could not verify with wrong env");
    let renv2 = lal::verify(&component_dir, &mf, "xenial".into(), true);
    assert!(
        renv2.is_err(),
        "could not verify with wrong env - even with simple"
    );

    let heylib = Path::new(&component_dir).join("INPUT").join("heylib");
    // clean folders and verify it fails
    fs::remove_dir_all(&heylib).unwrap();

    let r2 = lal::verify(&component_dir, &mf, env_name.into(), false);
    assert!(r2.is_err(), "verify failed after fiddling");

    // fetch --core, resyncs with core deps (removes devDeps and other extraneous)
    let rcore = lal::fetch(&component_dir, &mf, backend, true, env_name);
    assert!(rcore.is_ok(), "install core succeeded");
    assert!(heylib.is_dir(), "heylib was reinstalled from manifest");
    // TODO: add dev dep to verify it wasn't reinstalled here
    // assert!(!gtest.is_dir(), "gtest was was extraneous with --core => removed");

    // fetch --core also doesn't install else again
    let rcore2 = lal::fetch(&component_dir, &mf, backend, true, env_name);
    assert!(rcore2.is_ok(), "install core succeeded 2");
    assert!(heylib.is_dir(), "heylib still there");
    // assert!(!gtest.is_dir(), "gtest was not reinstalled with --core");

    // and it is finally installed if we ask for non-core as well
    let rall = lal::fetch(&component_dir, &mf, backend, false, env_name);
    assert!(rall.is_ok(), "install all succeeded");
    // assert!(gtest.is_dir(), "gtest is otherwise installed again");

    let r3 = lal::verify(&component_dir, &mf, env_name, false);
    assert!(r3.is_ok(), "verify ok again");
}

fn run_scripts(env_name: &str, home: &Path, component_dir: &Path) {
    {
        Command::new("mkdir")
            .arg("-p")
            .arg(".lal/scripts")
            .current_dir(&component_dir)
            .output()
            .unwrap();
        let mut f = File::create(&component_dir.join("./.lal/scripts/subroutine")).unwrap();
        write!(f, "main() {{ echo hi $1 $2 ;}}\n").unwrap();
        Command::new("chmod")
            .arg("+x")
            .arg(".lal/scripts/subroutine")
            .current_dir(&component_dir)
            .output()
            .unwrap();
    }
    let cfg = Config::read(Some(&home)).unwrap();
    let environment = cfg.get_environment(env_name.into()).unwrap();
    let modes = ShellModes::default();
    let r = lal::script(
        &cfg,
        &environment,
        "subroutine",
        vec!["there", "mr"],
        &modes,
        false,
        &component_dir,
    );
    assert!(r.is_ok(), "could run subroutine script");
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

fn status_on_experimentals(component_dir: &Path) {
    let mf = Manifest::read(&component_dir).unwrap();

    // both of these should return errors, but work
    let r = lal::status(&component_dir, &mf, false, false, false);
    assert!(r.is_err(), "status should complain at experimental deps");
    let r = lal::status(&component_dir, &mf, true, true, true);
    assert!(r.is_err(), "status should complain at experimental deps");
}

#[cfg(feature = "upgrade")]
fn upgrade_does_not_fail() {
    let uc = lal::upgrade(true);
    assert!(uc.is_ok(), "could perform upgrade check");
    let upgraded = uc.unwrap();
    assert!(!upgraded, "we never have upgrades in the tip source tree");
}

fn clean_check(home: &Path) {
    let cfg = Config::read(Some(&home)).unwrap();
    let r = lal::clean(&cfg.cache, 1);
    assert!(r.is_ok(), "could run partial lal cleanup");

    // scan cache dir
    let mut dirs = WalkDir::new(&cfg.cache)
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir());

    let first = dirs.next();
    assert!(first.is_some(), "some artifacts cached since last time");

    // run check again cleaning everything
    let r = lal::clean(&cfg.cache, 0);
    assert!(r.is_ok(), "could run full lal cleanup");

    // scan cache dir
    let mut dirs2 = WalkDir::new(&cfg.cache)
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir());

    let first2 = dirs2.next();
    assert!(first2.is_none(), "no artifacts left in cache");
}

fn export_check<T: CachedBackend + Backend>(env_name: &str, backend: &T, component_dir: &Path) {
    let tmp = component_dir.join("blah");
    debug!("tmp: {}", tmp.display());
    if !tmp.is_dir() {
        fs::create_dir(&tmp).unwrap();
    }

    let r = lal::export(backend, "heylib=1", &tmp, Some(env_name));
    assert!(r.is_ok(), "could export heylib=1 into subdir");

    let r2 = lal::export(backend, "hello", &tmp, Some(env_name));
    assert!(r2.is_ok(), "could export latest hello into subdir");

    let heylib = component_dir.join("blah").join("heylib.tar.gz");
    assert!(heylib.is_file(), "heylib was copied correctly");

    let hello = component_dir.join("blah").join("hello.tar.gz");
    assert!(hello.is_file(), "hello was copied correctly");

    // TODO: verify we can untar and execute hello binary and grep output after #15
}

fn query_check<T: Backend>(env_name: &str, backend: &T) {
    let r = lal::query(backend, Some(env_name), "hello", false);
    assert!(r.is_ok(), "could query for hello");

    let rl = lal::query(backend, Some(env_name), "hello", true);
    assert!(rl.is_ok(), "could query latest for hello");
}
