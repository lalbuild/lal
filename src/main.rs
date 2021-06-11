#[macro_use] extern crate clap;
#[macro_use] extern crate log;

use clap::ArgMatches;
use lal::{self, *};
use std::{env::current_dir, ops::Deref, path::Path, process};

fn result_exit<T>(name: &str, x: LalResult<T>) {
    let _ = x.map_err(|e| {
        println!(); // add a separator
        error!("{} error: {}", name, e);
        debug!("{}: {:?}", name, e); // in the off-chance that Debug is useful
        process::exit(1);
    });
    process::exit(0);
}

fn get_backend(config: &Config) -> Box<dyn CachedBackend> {
    match config.backend {
        BackendConfiguration::Artifactory(ref cfg) => Box::new(ArtifactoryBackend::new(&cfg, &config.cache)),
        BackendConfiguration::Local(ref cfg) => Box::new(LocalBackend::new(&cfg, &config.cache)),
    }
}

// functions that work without a manifest, and thus can run without a set env
fn handle_manifest_agnostic_cmds(
    args: &ArgMatches<'_>,
    cfg: &Config,
    component_dir: &Path,
    backend: &dyn CachedBackend,
    explicit_env: Option<&str>,
) {
    let res = if let Some(a) = args.subcommand_matches("export") {
        let curdir = current_dir().unwrap();
        lal::export(backend, a.value_of("component").unwrap(), &curdir, explicit_env)
    } else if let Some(a) = args.subcommand_matches("query") {
        lal::query(
            backend,
            explicit_env,
            a.value_of("component").unwrap(),
            a.is_present("latest"),
        )
    } else if let Some(a) = args.subcommand_matches("publish") {
        lal::publish(None, &component_dir, a.value_of("component").unwrap(), backend)
    } else if args.subcommand_matches("list-environments").is_some() {
        lal::list::environments(cfg)
    } else {
        return;
    };
    result_exit(args.subcommand_name().unwrap(), res);
}

// functions that need a manifest, but do not depend on environment values
fn handle_environment_agnostic_cmds(
    args: &ArgMatches<'_>,
    component_dir: &Path,
    mf: &Manifest,
    backend: &dyn CachedBackend,
) {
    let res = if let Some(a) = args.subcommand_matches("status") {
        lal::status(
            &component_dir,
            mf,
            a.is_present("full"),
            a.is_present("origin"),
            a.is_present("time"),
        )
    } else if args.subcommand_matches("list-components").is_some() {
        lal::list::buildables(mf)
    } else if args.subcommand_matches("list-supported-environments").is_some() {
        lal::list::supported_environments(mf)
    } else if let Some(a) = args.subcommand_matches("list-configurations") {
        lal::list::configurations(a.value_of("component").unwrap(), mf)
    } else if let Some(a) = args.subcommand_matches("list-dependencies") {
        lal::list::dependencies(mf, a.is_present("core"))
    } else if let Some(a) = args.subcommand_matches("remove") {
        let xs = a
            .values_of("components")
            .unwrap()
            .map(String::from)
            .collect::<Vec<_>>();
        lal::remove(
            &component_dir,
            mf,
            xs,
            a.is_present("save"),
            a.is_present("savedev"),
        )
    } else if let Some(a) = args.subcommand_matches("stash") {
        lal::stash(&component_dir, backend, mf, a.value_of("name").unwrap())
    } else if let Some(a) = args.subcommand_matches("propagate") {
        lal::propagate::print(
            &component_dir,
            mf,
            a.value_of("component").unwrap(),
            a.is_present("json"),
        )
    } else {
        return;
    };
    result_exit(args.subcommand_name().unwrap(), res);
}

fn handle_network_cmds(
    args: &ArgMatches<'_>,
    component_dir: &Path,
    mf: &Manifest,
    backend: &dyn CachedBackend,
    env: &str,
) {
    let res = if let Some(a) = args.subcommand_matches("update") {
        let xs = a
            .values_of("components")
            .unwrap()
            .map(String::from)
            .collect::<Vec<_>>();
        lal::update(
            &component_dir,
            mf,
            backend,
            xs,
            a.is_present("save"),
            a.is_present("savedev"),
            env,
        )
    } else if let Some(a) = args.subcommand_matches("update-all") {
        lal::update_all(
            &component_dir,
            mf,
            backend,
            a.is_present("save"),
            a.is_present("dev"),
            env,
        )
    } else if let Some(a) = args.subcommand_matches("fetch") {
        lal::fetch(&component_dir, mf, backend, a.is_present("core"), env)
    } else {
        return; // not a network cmnd
    };
    result_exit(args.subcommand_name().unwrap(), res)
}

fn handle_env_command(
    args: &ArgMatches<'_>,
    component_dir: &Path,
    cfg: &Config,
    mf: &Manifest,
    env: &str,
    stickies: &StickyOptions,
) -> Environment {
    // lookup associated container from
    let environment = mf.get_environment(env)
        .or_else(|_| cfg.get_environment(env))
        .map_err(|e| {
            error!("Environment error: {}", e);
            println!("Ensure that manifest.environment has a corresponding entry in ~/.lal/config");
            process::exit(1);
        })
        .unwrap();

    // resolve env updates and sticky options before main subcommands
    if let Some(a) = args.subcommand_matches("env") {
        if a.subcommand_matches("update").is_some() {
            result_exit("env update", lal::env::update(&component_dir, &environment, env))
        } else if a.subcommand_matches("reset").is_some() {
            // NB: if .lal/opts.env points at an environment not in config
            // reset will fail.. possible to fix, but complects this file too much
            // .lal/opts writes are checked in lal::env::set anyway so this
            // would be purely the users fault for editing it manually
            result_exit("env clear", lal::env::clear(&component_dir))
        } else if let Some(sa) = a.subcommand_matches("set") {
            result_exit(
                "env override",
                lal::env::set(&component_dir, stickies, cfg, mf, sa.value_of("environment").unwrap()),
            )
        } else {
            // just print current environment
            println!("{}", env);
            process::exit(0);
        }
    }
    // if we didn't handle an env subcommand here return the environment
    // needs to be resolved later on for docker cmds anyway
    environment
}

#[cfg(feature = "upgrade")]
fn handle_upgrade(args: &ArgMatches, cfg: &Config) {
    // we have a subcommand because SubcommandRequiredElseHelp
    let subname = args.subcommand_name().unwrap();

    // Allow lal upgrade without manifest
    if args.subcommand_matches("upgrade").is_some() {
        result_exit("upgrade", lal::upgrade(false)); // explicit, verbose check
    }

    // Autoupgrade if enabled - runs once daily if enabled
    // also excluding all listers because they are used in autocomplete
    if cfg.autoupgrade && subname != "upgrade" && !subname.contains("list-") && cfg.upgrade_check_time() {
        debug!("Performing daily upgrade check");
        let _ = lal::upgrade(false).map_err(|e| {
            error!("Daily upgrade check failed: {}", e);
            // don't halt here if this ever happens as it could break it for users
        });
        let _ = cfg.clone().performed_upgrade().map_err(|e| {
            error!("Daily upgrade check updating lastUpgrade failed: {}", e);
            // Ditto
        });
        debug!("Upgrade check done - continuing to requested operation\n");
    }
}

fn handle_docker_cmds(
    args: &ArgMatches<'_>,
    component_dir: &Path,
    mf: &Manifest,
    cfg: &Config,
    env: &str,
    environment: &Environment,
) {
    let res = if let Some(a) = args.subcommand_matches("verify") {
        // not really a docker related command, but it needs
        // the resolved env to verify consistent dependency usage
        lal::verify(&component_dir, mf, env, a.is_present("simple"))
    } else if let Some(a) = args.subcommand_matches("build") {
        let bopts = BuildOptions {
            name: a.value_of("component").map(String::from),
            configuration: a.value_of("configuration").map(String::from),
            release: a.is_present("release"),
            version: a.value_of("with-version").map(String::from),
            sha: a.value_of("with-sha").map(String::from),
            environment: environment.clone(),
            force: a.is_present("force"),
            simple_verify: a.is_present("simple-verify"),
        };
        let modes = ShellModes {
            printonly: a.is_present("print"),
            x11_forwarding: a.is_present("x11"),
            host_networking: a.is_present("net-host"),
            env_vars: values_t!(a.values_of("env-var"), String).unwrap_or_default(),
        };
        lal::build(&component_dir, cfg, mf, &bopts, env.into(), modes)
    } else if let Some(a) = args.subcommand_matches("shell") {
        let xs = if a.is_present("cmd") {
            Some(a.values_of("cmd").unwrap().collect::<Vec<_>>())
        } else {
            None
        };
        let modes = ShellModes {
            printonly: a.is_present("print"),
            x11_forwarding: a.is_present("x11"),
            host_networking: a.is_present("net-host"),
            env_vars: values_t!(a.values_of("env-var"), String).unwrap_or_default(),
        };
        lal::shell(
            cfg,
            environment,
            &modes,
            xs,
            a.is_present("privileged"),
            &component_dir,
        )
    } else if let Some(a) = args.subcommand_matches("run") {
        let xs: Vec<&str> = a.values_of("parameters")
            .map_or_else(Vec::new, |p| p.collect());

        let modes = ShellModes {
            printonly: a.is_present("print"),
            x11_forwarding: a.is_present("x11"),
            host_networking: a.is_present("net-host"),
            env_vars: values_t!(a.values_of("env-var"), String).unwrap_or_default(),
        };
        lal::script(
            cfg,
            environment,
            a.value_of("script").unwrap(),
            xs,
            &modes,
            a.is_present("privileged"),
            &component_dir,
        )
    } else {
        return; // no valid docker related command found
    };
    result_exit(args.subcommand_name().unwrap(), res);
}

fn main() {
    let app = lal::app::new();
    let args = app.get_matches();

    // by default, always show INFO messages for now (+1)
    loggerv::Logger::new()
        .verbosity(args.occurrences_of("verbose") + 1)
        .module_path(true)
        .line_numbers(args.is_present("debug"))
        .init()
        .unwrap();

    // Allow lal configure without assumptions
    if let Some(_a) = args.subcommand_matches("configure") {
        result_exit(
            "configure",
            lal::configure(true, true, None),
        );
    }

    // Force config to exists before allowing remaining actions
    let config = Config::read(None)
        .map_err(|e| {
            error!("Configuration error: {}", e);
            println!();
            println!("If you have just installed or upgraded, run `lal configure`");
            process::exit(1);
        })
        .unwrap();

    // Create a storage backend (something that implements storage/traits.rs)
    let backend: Box<dyn CachedBackend> = get_backend(&config);

    // Ensure SSL is initialized before using the backend
    openssl_probe::init_ssl_cert_env_vars();

    // Do upgrade checks or handle explicit `lal upgrade` here
    #[cfg(feature = "upgrade")]
    handle_upgrade(&args, &config);

    let component_dir = current_dir().unwrap();
    // Allow lal init / clean without manifest existing in PWD
    if let Some(a) = args.subcommand_matches("init") {
        result_exit(
            "init",
            lal::init(
                &config,
                a.is_present("force"),
                &component_dir,
                a.value_of("environment").unwrap(),
            ),
        );
    } else if let Some(a) = args.subcommand_matches("clean") {
        let days = a.value_of("days").unwrap().parse().unwrap();
        result_exit("clean", lal::clean(&config.cache, days));
    }

    // Read .lal/opts if it exists
    let stickies = StickyOptions::read(&component_dir)
        .map_err(|e| {
            // Should not happen unless people are mucking with it manually
            error!("Options error: {}", e);
            println!(".lal/opts must be valid json");
            process::exit(1);
        })
        .unwrap(); // we get a default empty options here otherwise

    // Manifest agnostic commands need explicit environments to not look in global location
    let explicit_env = args.value_of("environment");
    if let Some(env) = explicit_env {
        config
            .get_environment(env)
            .map_err(|e| {
                error!("Environment error: {}", e);
                process::exit(1)
            })
            .unwrap();
    }
    handle_manifest_agnostic_cmds(&args, &config, &component_dir, backend.deref(), explicit_env);

    // Force manifest to exist before allowing remaining actions
    let manifest = Manifest::read(&component_dir)
        .map_err(|e| {
            error!("Manifest error: {}", e);
            println!("Ensure manifest.json is valid json or run `lal init`");
            process::exit(1);
        })
        .unwrap();

    // Subcommands that are environment agnostic
    handle_environment_agnostic_cmds(&args, &component_dir, &manifest, backend.deref());

    // Force a valid container key configured in manifest and corr. value in config
    // NB: --env overrides sticky env overrides manifest.env
    let env = if let Some(eflag) = args.value_of("environment") {
        eflag.into()
    } else if let Some(ref stickenv) = stickies.env {
        stickenv.clone()
    } else {
        manifest.environment.clone()
    };
    let environment = handle_env_command(&args, &component_dir, &config, &manifest, &env, &stickies);

    // Warn users who are using an unsupported environment
    let sub = args.subcommand_name().unwrap();
    if !manifest
        .supportedEnvironments
        .clone()
        .into_iter()
        .any(|e| e == env)
    {
        warn!("Running {} command in unsupported {} environment", sub, env);
    } else {
        debug!("Running {} command in supported {} environent", sub, env);
    }

    // Main subcommands
    handle_network_cmds(&args, &component_dir, &manifest, backend.deref(), &env);
    handle_docker_cmds(&args, &component_dir, &manifest, &config, &env, &environment);

    unreachable!("Subcommand valid, but not implemented");
}
