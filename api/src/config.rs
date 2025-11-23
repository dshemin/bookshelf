use std::env;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Config {
    pub auth: AuthConfig,
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub auth_url: String,
    pub token_url: String,
    pub client_id: String,
    pub client_secret: Secret,
    pub redirect_url: String,
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

pub fn load() -> Config {
    let get = |name: &str| env::var(name).unwrap_or_default();

    Config {
        auth: AuthConfig {
            auth_url: get("BOOKSHELF_AUTH_URL"),
            token_url: get("BOOKSHELF_TOKEN_URL"),
            client_id: get("BOOKSHELF_CLIENT_ID"),
            client_secret: get("BOOKSHELF_CLIENT_SECRET").into(),
            redirect_url: get("BOOKSHELF_REDIRECT_URL"),
        },
    }
}
