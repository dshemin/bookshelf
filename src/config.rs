use anyhow::anyhow;
use dotenv::dotenv;
use serde::Deserialize;
use serde_with::with_prefix;

with_prefix!(http_config "http_");

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    #[serde(flatten, with = "http_config")]
    pub http: HTTPConfig,
    pub jwt_pub_key: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct HTTPConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8080
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
    envy::prefixed("BS_API_").from_env().map_err(|e| anyhow!(e))
}

type Result = anyhow::Result<Config>;
