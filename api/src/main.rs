mod config;
mod container;
mod endpoints;
mod telemetry;
mod version;

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth, Role};
use container::Container;
use tracing::info;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init(version::GIT_HASH).map_err(to_std_io_err)?;

    let cfg = config::collect().map_err(to_std_io_err)?;

    info!(config = tracing::field::debug(&cfg), "Started");

    let container = Container::new(&cfg).await.map_err(to_std_io_err)?;

    run_http_server(
        cfg.http.host,
        cfg.http.port,
        cfg.jwt_pub_key,
        cfg.enable_auth,
        cfg.cors.allowed_origin,
        container,
    )
    .await
}

async fn run_http_server(
    host: String,
    port: u16,
    jwt_pub_key: String,
    enable_auth: bool,
    allowed_origin: String,
    container: Container,
) -> std::io::Result<()> {
    let key = Box::new(DecodingKey::from_rsa_pem(jwt_pub_key.as_bytes()).map_err(to_std_io_err)?);

    HttpServer::new(move || {
        App::new()
            // .app_data(state.clone())
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(container.storage_create.clone()))
            .app_data(web::Data::new(container.storage_list.clone()))
            .app_data(web::Data::new(container.storage_get.clone()))
            .app_data(web::Data::new(container.storage_update.clone()))
            .app_data(web::Data::new(container.storage_delete.clone()))
            .configure(configure_api(
                enable_auth,
                *(key.clone()),
                allowed_origin.clone(),
            ))
            .service(endpoints::telemetry::healthz)
    })
    .bind((host, port))?
    .run()
    .await
}

fn to_std_io_err<E: ToString>(e: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
}

fn configure_api(
    enable_auth: bool,
    key: DecodingKey,
    allowed_origin: String,
) -> Box<dyn FnOnce(&mut web::ServiceConfig)> {
    Box::new(move |cfg: &mut web::ServiceConfig| {
        let default_headers =
            actix_web::middleware::DefaultHeaders::new().add(("Content-Type", "application/json"));

        let keycloak = setup_keycloak_middleware(enable_auth, key);

        let cors = setup_cors_middleware(&allowed_origin);

        let api = web::scope("/api")
            .wrap(default_headers)
            .wrap(cors)
            .wrap(keycloak)
            .service(setup_storages_endpoints());

        cfg.service(api);
    })
}

fn setup_keycloak_middleware(enable_auth: bool, key: DecodingKey) -> actix_web::middleware::Condition<KeycloakAuth<actix_web_middleware_keycloak_auth::AlwaysReturnPolicy>> {
    let keycloak_auth_admin = {
        let mut auth = KeycloakAuth::default_with_pk(key);
        auth.required_roles = vec![Role::Realm {
            role: "Realm admin".to_owned(),
        }];
        auth
    };

    actix_web::middleware::Condition::new(
        enable_auth,
        keycloak_auth_admin,
    )
}

fn setup_cors_middleware(allowed_origin: &str) -> Cors {
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .supports_credentials();

    if allowed_origin == "*" {
        cors = cors.allow_any_origin();
    } else {
        cors = cors.allowed_origin(allowed_origin);
    }

    cors
}

fn setup_storages_endpoints() -> actix_web::Scope {
    web::scope("/storages")
        .service(endpoints::storages::create)
        .service(endpoints::storages::list)
        .service(endpoints::storages::get)
        .service(endpoints::storages::update)
        .service(endpoints::storages::delete)
        .service(endpoints::storages::upload_file)
}
