use clap::{crate_version, App, AppSettings, Arg, SubCommand};

fn is_integer(v: String) -> Result<(), String> {
    if v.parse::<u32>().is_ok() {
        return Ok(());
    }
    Err(format!("{} is not an integer", v))
}

/// lal clap app
pub fn new<'a>() -> App<'a, 'a> {
    #[rustfmt::skip]
    let mut app = App::new("lal")
        .version(crate_version!())
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .global_settings(&[AppSettings::ColoredHelp])
        .about("lal dependency manager")
        .arg(Arg::with_name("environment")
            .short("e")
            .long("env")
            .takes_value(true)
            .help("Override the default environment for this command"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Increase verbosity"))
       .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Adds line numbers to log statements"))
        .subcommand(SubCommand::with_name("fetch")
            .about("Fetch dependencies listed in the manifest into INPUT")
            .arg(Arg::with_name("core")
                .long("core")
                .short("c")
                .help("Only fetch core dependencies")))
        .subcommand(SubCommand::with_name("build")
            .about("Runs BUILD script in current directory in the configured container")
            .arg(Arg::with_name("component")
                .help("Build a specific component (if other than the main manifest component)"))
            .arg(Arg::with_name("configuration")
                .long("config")
                .short("c")
                .takes_value(true)
                .help("Build using a specific configuration (else will use defaultConfig)"))
            .arg(Arg::with_name("simple-verify")
                .short("s")
                .long("simple-verify")
                .help("Use verify --simple to check INPUT (allows stashed dependencies)"))
            .arg(Arg::with_name("force")
                .long("force")
                .short("f")
                .help("Ignore verify errors when using custom dependencies"))
            .arg(Arg::with_name("release")
                .long("release")
                .short("r")
                .help("Create a release tarball that can be published"))
            .arg(Arg::with_name("with-version")
                .long("with-version")
                .takes_value(true)
                .requires("release")
                .help("Configure lockfiles with an explicit version number"))
            .arg(Arg::with_name("with-sha")
                .long("with-sha")
                .takes_value(true)
                .requires("release")
                .help("Configure lockfiles with an explicit sha"))
            .arg(Arg::with_name("x11")
                .short("X")
                .long("X11")
                .help("Enable best effort X11 forwarding"))
            .arg(Arg::with_name("net-host")
                .short("n")
                .long("net-host")
                .help("Enable host networking"))
            .arg(Arg::with_name("env-var")
                .long("env-var")
                .help("Set environment variables in the container")
                .multiple(true)
                .takes_value(true)
                .number_of_values(1))
            .arg(Arg::with_name("print")
                .long("print-only")
                .conflicts_with("release")
                .help("Only print the docker run command and exit")))
        .subcommand(SubCommand::with_name("update")
            .about("Update arbitrary dependencies into INPUT")
            .arg(Arg::with_name("components")
                .help("The specific component=version pairs to update")
                .required(true)
                .multiple(true))
            .arg(Arg::with_name("save")
                .short("S")
                .long("save")
                .conflicts_with("savedev")
                .help("Save updated versions in dependencies in the manifest"))
            .arg(Arg::with_name("savedev")
                .short("D")
                .long("save-dev")
                .conflicts_with("save")
                .help("Save updated versions in devDependencies in the manifest")))
        .subcommand(SubCommand::with_name("verify")
            .arg(Arg::with_name("simple")
                .short("s")
                .long("simple")
                .help("Allow stashed versions in this simpler verify algorithm"))
            .about("verify consistency of INPUT"))
        .subcommand(SubCommand::with_name("status")
            .alias("ls")
            .arg(Arg::with_name("full")
                .short("f")
                .long("full")
                .help("Print the full dependency tree"))
            .arg(Arg::with_name("time")
                .short("t")
                .long("time")
                .help("Print build time of artifact"))
            .arg(Arg::with_name("origin")
                .short("o")
                .long("origin")
                .help("Print version and environment origin of artifact"))
            .about("Prints current dependencies and their status"))
        .subcommand(SubCommand::with_name("shell")
            .about("Enters the configured container mounting the current directory")
            .alias("sh")
            .arg(Arg::with_name("privileged")
                .short("p")
                .long("privileged")
                .help("Run docker in privileged mode"))
            .arg(Arg::with_name("x11")
                .short("X")
                .long("X11")
                .help("Enable X11 forwarding (best effort)"))
            .arg(Arg::with_name("net-host")
                .short("n")
                .long("net-host")
                .help("Enable host networking"))
            .arg(Arg::with_name("env-var")
                .long("env-var")
                .help("Set environment variables in the container")
                .multiple(true)
                .takes_value(true)
                .number_of_values(1))
            .arg(Arg::with_name("print")
                .long("print-only")
                .help("Only print the docker run command and exit"))
            .setting(AppSettings::TrailingVarArg)
            .arg(Arg::with_name("cmd").multiple(true)))
        .subcommand(SubCommand::with_name("run")
            .about("Runs scripts from .lal/scripts in the configured container")
            .alias("script")
            .arg(Arg::with_name("script")
                .help("Name of the script file to be run")
                .required(true))
            .arg(Arg::with_name("x11")
                .short("X")
                .long("X11")
                .help("Enable X11 forwarding (best effort)"))
            .arg(Arg::with_name("net-host")
                .short("n")
                .long("net-host")
                .help("Enable host networking"))
            .arg(Arg::with_name("env-var")
                .long("env-var")
                .help("Set environment variables in the container")
                .multiple(true)
                .takes_value(true)
                .number_of_values(1))
            .arg(Arg::with_name("print")
                .long("print-only")
                .help("Only print the docker run command and exit"))
            .arg(Arg::with_name("privileged")
                .short("p")
                .long("privileged")
                .help("Run docker in privileged mode"))
            .setting(AppSettings::TrailingVarArg)
            .arg(Arg::with_name("parameters")
                .multiple(true)
                .help("Parameters to pass on to the script")))
        .subcommand(SubCommand::with_name("init")
            .about("Create a manifest file in the current directory")
            .arg(Arg::with_name("environment")
                .required(true)
                .help("Environment to build this component in"))
            .arg(Arg::with_name("force")
                .short("f")
                .help("overwrites manifest if necessary")))
        .subcommand(SubCommand::with_name("configure")
            .about("Creates a default lal config ~/.lal/ from a defaults file")
            .arg(Arg::with_name("file")
                .required(true)
                .help("An environments file to seed the config with")))
        .subcommand(SubCommand::with_name("export")
            .about("Fetch a raw tarball from artifactory")
            .arg(Arg::with_name("component")
                .help("The component to export")
                .required(true))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Output directory to save to")))
        .subcommand(SubCommand::with_name("env")
            .about("Manages environment configurations")
            .subcommand(SubCommand::with_name("set")
                .about("Override the default environment for this folder")
                .arg(Arg::with_name("environment")
                    .required(true)
                    .help("Name of the environment to use")))
            .subcommand(SubCommand::with_name("update").about("Update the current environment"))
            .subcommand(SubCommand::with_name("reset").about("Return to the default environment")))
        .subcommand(SubCommand::with_name("stash")
            .about("Stashes current build OUTPUT in cache for later reuse")
            .alias("save")
            .arg(Arg::with_name("name")
                .required(true)
                .help("Name used for current build")))
        .subcommand(SubCommand::with_name("remove")
            .alias("rm")
            .about("Remove specific dependencies from INPUT")
            .arg(Arg::with_name("components")
                .help("Remove specific components")
                .required(true)
                .multiple(true))
            .arg(Arg::with_name("save")
                .short("S")
                .long("save")
                .conflicts_with("savedev")
                .help("Save removal of dependencies in the manifest"))
            .arg(Arg::with_name("savedev")
                .short("D")
                .long("save-dev")
                .conflicts_with("save")
                .help("Save removal of devDependencies in the manifest")))
        .subcommand(SubCommand::with_name("clean")
            .about("Clean old artifacts in the cache directory to save space")
            .arg(Arg::with_name("days")
                .short("d")
                .long("days")
                .takes_value(true)
                .default_value("14")
                .validator(is_integer)
                .help("Number of days to serve as cutoff")))
        .subcommand(SubCommand::with_name("query")
            .about("Query for available versions on artifactory")
            .arg(Arg::with_name("latest")
                .long("latest")
                .short("l")
                .help("Return latest version only"))
            .arg(Arg::with_name("component")
                .required(true)
                .help("Component name to search for")))
        .subcommand(SubCommand::with_name("propagate")
            .about("Show steps to propagate a version fully through the tree")
            .arg(Arg::with_name("component")
                .required(true)
                .help("Component to propagate"))
            .arg(Arg::with_name("json")
                .short("j")
                .long("json")
                .help("Produce a machine readable instruction set")))
        .subcommand(SubCommand::with_name("update-all")
            .about("Update all dependencies in the manifest")
            .arg(Arg::with_name("dev")
                .short("D")
                .long("dev")
                .help("Update devDependencies instead of dependencies"))
            .arg(Arg::with_name("save")
                .short("S")
                .long("save")
                .help("Save updated versions in the right object in the manifest")))
        .subcommand(SubCommand::with_name("publish")
            .setting(AppSettings::Hidden)
            .arg(Arg::with_name("component")
                .required(true)
                .help("Component name to publish"))
            .about("Publish a release build to the default artifactory location"))
        .subcommand(SubCommand::with_name("list-components")
            .setting(AppSettings::Hidden)
            .about("list components that can be used with lal build"))
        .subcommand(SubCommand::with_name("list-supported-environments")
            .setting(AppSettings::Hidden)
            .about("list supported environments from the manifest"))
        .subcommand(SubCommand::with_name("list-environments")
            .setting(AppSettings::Hidden)
            .about("list environments that can be used with lal build"))
        .subcommand(SubCommand::with_name("list-configurations")
            .setting(AppSettings::Hidden)
            .arg(Arg::with_name("component")
                .required(true)
                .help("Component name to look for in the manifest"))
            .about("list configurations for a given component"))
        .subcommand(SubCommand::with_name("list-dependencies")
            .setting(AppSettings::Hidden)
            .arg(Arg::with_name("core")
                .short("c")
                .long("core")
                .help("Only list core dependencies"))
            .about("list dependencies from the manifest"));

    if cfg!(feature = "upgrade") {
        app = app
            .subcommand(SubCommand::with_name("upgrade").about("Attempts to upgrade lal from artifactory"));
    }

    return app;
}
