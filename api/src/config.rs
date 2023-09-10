use anyhow::anyhow;
use dotenv::dotenv;
use serde::Deserialize;
use serde_with::{with_prefix, serde_as, DisplayFromStr};

with_prefix!(http_config "http_");
with_prefix!(pg_config "pg_");

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default = "default_enable_auth")]
    pub enable_auth: bool,
    #[serde(flatten, with = "http_config")]
    pub http: HTTPConfig,
    pub jwt_pub_key: String,
    #[serde(flatten, with = "pg_config")]
    pub pg: PGConfig,
}

fn default_enable_auth() -> bool {
    true
}

#[serde_as]
#[derive(Deserialize, Debug, Default)]
pub struct HTTPConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    80
}

#[derive(Deserialize, Debug, Default)]
pub struct PGConfig {
    #[serde(default = "default_conn_uri")]
    pub conn_uri: String,
}

fn default_conn_uri() -> String {
    "postgres://postgres:123456@127.0.0.1/bookshelf".to_string()
}

// Collects all configuration parameters.
pub fn collect() -> Result {
    loadenv()?;
    fill()
}

fn loadenv() -> anyhow::Result<()> {
    use std::io::ErrorKind::NotFound;

    match dotenv() {
        // Dotenv file might not present at all.
        Err(dotenv::Error::Io(e)) if e.kind() == NotFound => Ok(()),
        Err(e) => Err(anyhow!(e)),
        Ok(_) => Ok(()),
    }
}

fn fill() -> Result {
    let cfg = envy::prefixed("BS_API_").from_env()?;
    Ok(cfg)
}

type Result = anyhow::Result<Config>;
