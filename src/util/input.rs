use std::fs;
use std::path::Path;
use std::env;
use std::collections::HashMap;

use init::Manifest;
use errors::LalResult;

pub fn analyze() -> LalResult<HashMap<String, String>> {
    let cwd = try!(env::current_dir());
    let input = Path::new(&cwd).join("INPUT");

    let mut deps = HashMap::new();
    if !input.is_dir() {
        return Ok(deps);
    }

    for entry in try!(fs::read_dir(&input)) {
        let pth = try!(entry).path();
        if pth.is_dir() {
            let component = pth.to_str().unwrap().split("/").last().unwrap();
            // TODO: read version from lockfile
            deps.insert(component.to_string(), "experimental".to_string());
        }
    }
    Ok(deps)
}

pub struct InputDependency {
    pub name: String,
    pub missing: bool,
    pub extraneous: bool,
    pub development: bool,
    pub version: String, // on disk
    pub requirement: Option<String>, // from manifest
}

pub type InputMap = HashMap<String, InputDependency>;


pub fn analyze_full(manifest: &Manifest) -> LalResult<InputMap> {
    let cwd = try!(env::current_dir());
    let input = Path::new(&cwd).join("INPUT");

    let deps = try!(analyze());
    let saved_deps = manifest.all_dependencies();

    let mut depmap = InputMap::new();
    if !input.is_dir() {
        return Ok(depmap);
    }

    // check manifested deps
    for (d, v) in saved_deps.clone() {
        depmap.insert(d.clone(),
                      InputDependency {
                          name: d.clone(),
                          version: "experimental".to_string(), // TODO: from lockfile
                          requirement: Some(format!("{}", v)),
                          missing: deps.get(&d).is_none(),
                          development: manifest.devDependencies.contains_key(&d),
                          extraneous: false,
                      });
    }
    // check for potentially non-manifested deps
    for name in deps.keys() {
        if !saved_deps.contains_key(name) {
            depmap.insert(name.clone(),
                          InputDependency {
                              name: name.clone(),
                              version: "experimental".to_string(), // TODO: from lockfile!
                              requirement: None,
                              missing: false,
                              development: false,
                              extraneous: true,
                          });
        }
    }

    Ok(depmap)
}
