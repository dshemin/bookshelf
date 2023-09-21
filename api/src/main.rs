mod config;
mod endpoints;
mod telemetry;
mod version;

use actix_web::{web, App, HttpServer};
use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth, Role};
use application::storage::{repository as storage_repository, service as storage_services};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tracing::info;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init(version::GIT_HASH).map_err(to_std_io_err)?;

    let cfg = config::collect().map_err(to_std_io_err)?;

    info!(config = tracing::field::debug(&cfg), "Started");

    let pool = PgPoolOptions::new()
        .connect(&cfg.pg.conn_uri)
        .await
        .map_err(to_std_io_err)?;

    let state = AppStateInner::new(&pool);

    run_http_server(
        cfg.http.host,
        cfg.http.port,
        cfg.jwt_pub_key,
        cfg.enable_auth,
        state,
    )
    .await
}

#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub storage_services: Arc<StorageServices>,
}

impl AppStateInner {
    fn new(pool: &PgPool) -> Self {
        let storage_services = Arc::new(StorageServices::new(pool));

        Self { storage_services }
    }
}

#[derive(Debug)]
pub struct StorageServices {
    pub create: storage_services::Create,
    pub list: storage_services::List,
    pub get: storage_services::Get,
    pub update: storage_services::Update,
    pub delete: storage_services::Delete,
}

impl StorageServices {
    fn new(pool: &PgPool) -> Self {
        let repository = Box::new(storage_repository::pg::Repository::new(pool.clone()));

        let create = storage_services::Create::new(repository.clone());
        let list = storage_services::List::new(repository.clone());
        let get = storage_services::Get::new(repository.clone());
        let update = storage_services::Update::new(repository.clone());
        let delete = storage_services::Delete::new(repository);

        Self {
            create,
            list,
            get,
            update,
            delete,
        }
    }
}

pub type AppState = web::Data<AppStateInner>;

async fn run_http_server(
    host: String,
    port: u16,
    jwt_pub_key: String,
    enable_auth: bool,
    state: AppStateInner,
) -> std::io::Result<()> {
    let key = Box::new(DecodingKey::from_rsa_pem(jwt_pub_key.as_bytes()).map_err(to_std_io_err)?);

    let state = AppState::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(TracingLogger::default())
            .configure(configure_api(enable_auth, *(key.clone())))
            .service(endpoints::telemetry::healthz)
    })
    .bind((host, port))?
    .run()
    .await
}

fn to_std_io_err<E: ToString>(e: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
}

fn configure_api(enable_auth: bool, key: DecodingKey) -> Box<dyn FnOnce(&mut web::ServiceConfig)> {
    Box::new(move |cfg: &mut web::ServiceConfig| {
        let default_headers =
            actix_web::middleware::DefaultHeaders::new().add(("Content-Type", "application/json"));

        let keycloak_auth_admin = {
            let mut auth = KeycloakAuth::default_with_pk(key);
            auth.required_roles = vec![Role::Realm {
                role: "Realm admin".to_owned(),
            }];
            auth
        };

        let api = web::scope("/api")
            .wrap(default_headers)
            .wrap(actix_web::middleware::Condition::new(
                enable_auth,
                keycloak_auth_admin,
            ))
            .service(setup_storages_endpoints());

        cfg.service(api);
    })
}

fn setup_storages_endpoints() -> actix_web::Scope {
    web::scope("/storages")
        .service(endpoints::storages::create)
        .service(endpoints::storages::list)
        .service(endpoints::storages::get)
        .service(endpoints::storages::update)
        .service(endpoints::storages::delete)
}
