mod config;

use axum::{Router, routing::get};
use log::info;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cfg = config::load();

    let state = AppState {
        config: cfg.clone(),
        state: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/healthz", get(|| async { "Ok" }))
        .route("/readiness", get(|| async { "Ok" }))
        .with_state(state);

    info!(config:? = &cfg; "starting server...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    config: config::Config,
    state: Arc<Mutex<HashMap<String, String>>>,
}
