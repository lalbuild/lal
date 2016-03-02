use std::io;
use rustc_serialize::json;
use std::path::Path;
use std::fs::File;
use std::env;
use std::io::prelude::*;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Config {
    pub registry: String,
    pub cache: String,
    pub target: String,
    pub container: String,
}

fn prompt(name: &str, default: String) -> String {
    use std::io::{self, Write};
    print!("Default {}: ({}) ", name, &default);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n > 1 {
                // more than just a newline character (which we strip)
                return (&input[0..n - 1]).to_string();
            }
        }
        Err(error) => println!("error: {}", error),
    }
    return default;
}

pub fn current_config() -> io::Result<Config> {
    let home = env::home_dir().unwrap(); // crash if no $HOME
    let cfg_path = Path::new(&home).join(".lal/lalrc");
    if !cfg_path.exists() {
        panic!("You need to run `lal configure` to create `lalrc` \
            before using other commands.");
    }
    let mut f = try!(File::open(&cfg_path));
    let mut cfg_str = String::new();
    try!(f.read_to_string(&mut cfg_str));
    return Ok(json::decode(&cfg_str).unwrap());
}

pub fn configure() -> io::Result<Config> {
    let mut cfg = Config {
        registry: "http://localhost".to_string(),
        cache: "~/.lal/cache".to_string(),
        target: "ncp.amd64".to_string(),
        container: "edonusdevelopers/centos_build".to_string(),
    };

    let home = env::home_dir().unwrap(); // crash if no $HOME
    let cfg_path = Path::new(&home).join(".lal/lalrc");

    // Prompt for values:
    cfg.registry = prompt("registry", cfg.registry);
    cfg.cache = prompt("cache", cfg.cache);
    cfg.target = prompt("target", cfg.target);
    cfg.container = prompt("container", cfg.container);

    // Encode
    let encoded = json::as_pretty_json(&cfg);

    let mut f = try!(File::create(&cfg_path));
    try!(write!(f, "{}\n", encoded));

    println!("Wrote config {}: \n{}", cfg_path.display(), encoded);

    // TODO: check that docker is present and warn if not
    // TODO: check that docker images contains cfg.container and provide info if not
    return Ok(cfg.clone());
}