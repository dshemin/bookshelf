use std::collections::HashMap;

use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::layer::SubscriberExt;

/// Initialize application telemetry.
pub fn init(commit_hash: &str) -> Result {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("BS_API_LOG")
        .from_env()?;

    let default_fields = {
        let mut m = HashMap::new();
        m.insert("commit_hash".into(), commit_hash.into());
        m
    };

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::with_default_fields(
            "Bookshelf".into(),
            std::io::stdout,
            default_fields,
        ));

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

pub type Result = anyhow::Result<()>;
