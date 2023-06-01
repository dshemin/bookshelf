mod config;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth};
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::layer::SubscriberExt;

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}

#[get("")]
async fn shelfs_list() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_telemetry().map_err(to_std_io_err)?;

    let cfg = config::collect().map_err(to_std_io_err)?;

    info!(config = tracing::field::debug(&cfg), "Started");

    run_http_server(cfg.http.host, cfg.http.port, cfg.jwt_pub_key).await
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

async fn run_http_server(host: String, port: u16, jwt_pub_key: String) -> std::io::Result<()> {
    let key = Box::new(DecodingKey::from_rsa_pem(jwt_pub_key.as_bytes()).map_err(to_std_io_err)?);

    HttpServer::new(move || {
        let keycloak_auth = KeycloakAuth::default_with_pk(*(key.clone()));

        App::new()
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api")
                    .wrap(keycloak_auth)
                    .service(web::scope("/shelfs").service(shelfs_list)),
            )
            .service(healthz)
    })
    .bind((host, port))?
    .run()
    .await
}

fn to_std_io_err<E: ToString>(e: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
}
