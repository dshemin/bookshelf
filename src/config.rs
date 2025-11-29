use std::env;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Config {}

/// Special type to hide some sensative information during format.
#[derive(Clone)]
pub struct Secret(String);

pub fn load() -> Config {
    let get = |name: &str| env::var(name).unwrap_or_default();

    Config {}
}

impl From<Secret> for String {
    fn from(val: Secret) -> String {
        val.0
    }
}

impl From<String> for Secret {
    fn from(value: String) -> Self {
        Secret(value)
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "*******")
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "*******")
    }
}
