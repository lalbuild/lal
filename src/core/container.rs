use std::fmt;

/// Representation of a docker container image
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Container {
    /// The fully qualified image name
    pub name: String,
    /// The tag to use
    pub tag: String,
}

impl Container {
    /// Container struct with latest tag
    pub fn latest(name: &str) -> Self {
        Container {
            name: name.into(),
            tag: "latest".into(),
        }
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.tag)
    }
}

/// Convenience default for functions that require Lockfile inspection
/// Intentionally kept distinct from normal build images
impl Default for Container {
    fn default() -> Self {
        Container {
            name: "ubuntu".into(),
            tag: "xenial".into(),
        }
    }
}

impl Container {
    /// Initialize a container struct
    ///
    /// This will split the container on `:` to actually fetch the tag, and if no tag
    /// was present, it will assume tag is latest as per docker conventions.
    pub fn new(container: &str) -> Container {
        let split: Vec<&str> = container.split(':').collect();
        let tag = if split.len() == 2 { split[1] } else { "latest" };
        let cname = if split.len() == 2 { split[0] } else { container };
        Container {
            name: cname.into(),
            tag: tag.into(),
        }
    }
}
