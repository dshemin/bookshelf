mod config;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::layer::SubscriberExt;

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_telemetry().map_err(to_std_io_err)?;

    let cfg = config::collect().map_err(to_std_io_err)?;

    info!("Started");

    HttpServer::new(move || App::new().wrap(TracingLogger::default()).service(healthz))
        .bind((cfg.http.host, cfg.http.port))?
        .run()
        .await
}

fn init_telemetry() -> anyhow::Result<()> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("BS_API_LOG")
        .from_env()?;

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(
            "Bookshelf".into(),
            std::io::stdout,
        ));

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn to_std_io_err(e: anyhow::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
}
