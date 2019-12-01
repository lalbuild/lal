use std::fmt;

use super::Container;

/// Representation of a possible command execution environment
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Environment {
    /// A Docker container environment.
    Container(Container),
    /// No environment, use what is already on the host.
    None,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Container(container) => write!(f, "{}", container),
            Environment::None => write!(f, "No environment"),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::None
    }
}
