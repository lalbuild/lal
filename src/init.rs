use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use std::vec::Vec;
use rustc_serialize::json;

use errors::{CliError, LalResult};

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub components: HashMap<String, String>,
    pub flags: Vec<String>,
    pub dependencies: HashMap<String, u32>,
    pub devDependencies: HashMap<String, u32>,
}

// helper fn
pub fn merge_dependencies(m: &Manifest) -> HashMap<String, u32> {
    // create the joined hashmap of dependencies and possibly devdependencies
    let mut deps = m.dependencies.clone();
    for (k, v) in &m.devDependencies {
        deps.insert(k.clone(), v.clone());
    }
    deps
}

pub fn read_manifest() -> LalResult<Manifest> {
    let pwd = env::current_dir().unwrap();
    let manifest_path = Path::new(&pwd).join("manifest.json");
    if !manifest_path.exists() {
        return Err(CliError::MissingManifest);
    }
    let mut f = try!(File::open(&manifest_path));
    let mut manifest_str = String::new();
    try!(f.read_to_string(&mut manifest_str));
    let res = try!(json::decode(&manifest_str));
    Ok(res)
}

pub fn save_manifest(m: &Manifest) -> LalResult<()> {
    let pwd = env::current_dir().unwrap();
    let encoded = json::as_pretty_json(&m);

    let manifest_path = Path::new(&pwd).join("manifest.json");
    let mut f = try!(File::create(&manifest_path));
    try!(write!(f, "{}\n", encoded));
    info!("Wrote manifest {}: \n{}", manifest_path.display(), encoded);
    Ok(())
}

pub fn init(force: bool) -> LalResult<()> {
    let pwd = env::current_dir().unwrap();
    let last_comp = pwd.components().last().unwrap(); // std::path::Component
    let dirname = last_comp.as_os_str().to_str().unwrap();

    let manifest = Manifest {
        name: dirname.to_string(),
        version: "0".to_string(),
        components: HashMap::new(),
        flags: vec![],
        dependencies: HashMap::new(),
        devDependencies: HashMap::new(),
    };

    let encoded = json::as_pretty_json(&manifest);

    let manifest_path = Path::new(&pwd).join("manifest.json");
    if !force && manifest_path.exists() {
        return Err(CliError::ManifestExists);
    }
    let mut f = try!(File::create(&manifest_path));
    try!(write!(f, "{}\n", encoded));

    info!("Wrote manifest {}: \n{}", manifest_path.display(), encoded);
    Ok(())
}
