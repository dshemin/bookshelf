//! Holds application configuration.
//!

use anyhow::anyhow;
use dotenv::dotenv;
use serde::Deserialize;
use serde_with::{serde_as, with_prefix, DisplayFromStr};

with_prefix!(http_config "http_");
with_prefix!(cors_config "cors_");
with_prefix!(pg_config "pg_");

/// The application configuration.
#[derive(Deserialize, Debug, Default)]
pub struct Config {
    /// Authorization will be enabled if true.
    /// Otherwise no authorization will be applied.
    #[serde(default = "default_enable_auth")]
    pub enable_auth: bool,

    /// Configuration for HTTP server.
    #[serde(flatten, with = "http_config")]
    pub http: HTTPConfig,

    /// CORS configuration.
    #[serde(flatten, with = "cors_config")]
    pub cors: CORSConfig,

    /// Public key for decrypting JWT tokens.
    pub jwt_pub_key: String,

    /// Configuration for PostgreSQL connection.
    #[serde(flatten, with = "pg_config")]
    pub pg: PGConfig,
}

fn default_enable_auth() -> bool {
    true
}

/// Configuration for HTTP server.
#[serde_as]
#[derive(Deserialize, Debug, Default)]
pub struct HTTPConfig {
    /// The host to bind to.
    #[serde(default = "default_host")]
    pub host: String,

    /// The port to bind to.
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

/// CORS configuration.
#[serde_as]
#[derive(Deserialize, Debug, Default)]
pub struct CORSConfig {
    /// Allowed origin.
    #[serde(default = "default_allowed_origin")]
    pub allowed_origin: String,
}

fn default_allowed_origin() -> String {
    "*".to_string()
}

/// Configuration for PostgreSQL connection.
#[derive(Deserialize, Debug, Default)]
pub struct PGConfig {
    /// The connection URI.
    /// https://www.postgresql.org/docs/current/libpq-connect.html#id-1.7.3.8.3.6
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
