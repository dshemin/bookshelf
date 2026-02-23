mod config;

use axum::{Router, routing::get};
use log::info;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();

    let cfg = config::load()?;

    let state = AppState {
        config: cfg.clone(),
    };

    let app = Router::new()
        .route("/healthz", get(|| async { "Ok" }))
        .route("/readiness", get(|| async { "Ok" }))
        .with_state(state);

    info!(config:? = &cfg; "starting server");
    let listener = tokio::net::TcpListener::bind(cfg.address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn setup_logger() {
    env_logger::builder()
        .parse_env("BOOKSHELF_LOG")
        .filter_level(log::LevelFilter::Info)
        .format_file(true)
        .format_line_number(true)
        .init();
}

#[derive(Clone)]
struct AppState {
    config: config::Config,
}
