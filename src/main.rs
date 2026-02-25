mod config;
mod schema;
mod sqlite;

use axum::{Router, routing::get};
use log::info;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();

    let cfg = config::load()?;
    let db_url: String = cfg.db.clone().into();

    let state = AppState {
        config: cfg.clone(),
        db: sqlite::connect(&db_url).await?,
    };

    let app = Router::new()
        .route("/healthz", get(|| async { "Ok" }))
        .route("/readiness", get(|| async { "Ok" }))
        .with_state(state);

    info!(config:? = &cfg; "start server");
    let listener = tokio::net::TcpListener::bind(cfg.address).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
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

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("can't install CTRL-C signal");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => { info!("stop server") },
        _ = terminate => { info!("stop server") },
    }
}

#[derive(Clone)]
struct AppState {
    config: config::Config,
    db: sqlite::ConnectionPool,
}
