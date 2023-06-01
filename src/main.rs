mod config;
mod telemetry;
mod version;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth};
use tracing::info;
use tracing_actix_web::TracingLogger;

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
    telemetry::init(version::GIT_HASH).map_err(to_std_io_err)?;

    let cfg = config::collect().map_err(to_std_io_err)?;

    info!(config = tracing::field::debug(&cfg), "Started");

    run_http_server(cfg.http.host, cfg.http.port, cfg.jwt_pub_key).await
}

async fn run_http_server(host: String, port: u16, jwt_pub_key: String) -> std::io::Result<()> {
    let key = Box::new(DecodingKey::from_rsa_pem(jwt_pub_key.as_bytes()).map_err(to_std_io_err)?);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(configure_api(*(key.clone())))
            .service(healthz)
    })
    .bind((host, port))?
    .run()
    .await
}

fn to_std_io_err<E: ToString>(e: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
}

fn configure_api(key: DecodingKey) -> Box<dyn FnOnce(&mut web::ServiceConfig)> {
    let keycloak_auth = KeycloakAuth::default_with_pk(key);

    Box::new(|cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .wrap(keycloak_auth)
                .service(web::scope("/shelfs").service(shelfs_list)),
        );
    })
}
