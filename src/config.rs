use std::env;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
    pub db: Secret,
}

pub fn load() -> dotenvy::Result<Config> {
    match dotenvy::dotenv_override() {
        Ok(_) => Ok(()),
        // It's okay if there is no .env file.
        Err(dotenvy::Error::Io(ref e)) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }?;

    let get = |name: &str| env::var(name).unwrap_or_default();

    Ok(Config {
        address: get("BOOKSHELF_ADDRESS"),
        db: get("BOOKSHELF_DATABASE_URL").into(),
    })
}

/// Special type to hide some sensative information during format.
#[derive(Clone)]
pub struct Secret(String);

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
