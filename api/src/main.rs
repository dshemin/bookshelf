mod config;
mod telemetry;
mod version;
mod endpoints;

use sqlx::postgres::{PgPool, PgPoolOptions};
use actix_web::{web, App, HttpServer};
use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth};
use tracing::info;
use tracing_actix_web::TracingLogger;
use application::user::{service as user_services, repository as user_repository};
use application::storage::{service as storage_services, repository as storage_repository, self};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init(version::GIT_HASH).map_err(to_std_io_err)?;

    let cfg = config::collect().map_err(to_std_io_err)?;

    info!(config = tracing::field::debug(&cfg), "Started");

    let pool = PgPoolOptions::new().connect(&cfg.pg.conn_uri).await.map_err(to_std_io_err)?;

    let state = AppStateInner::new(&pool);

    run_http_server(cfg.http.host, cfg.http.port, cfg.jwt_pub_key, state).await
}

#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub user_services: Arc<UserServices>,
    pub storage_services: Arc<StorageServices>,
}

impl AppStateInner {
    fn new(pool: &PgPool) -> Self {
        let user_services = Arc::new(UserServices::new(pool));
        let storage_services = Arc::new(StorageServices::new(pool));

        Self {
            user_services,
            storage_services
        }
    }
}

#[derive(Debug)]
pub struct UserServices{
    pub sync: Arc<user_services::Sync>,
}

impl UserServices {
    fn new(pool: &PgPool) -> Self {
        let repository = Box::new(user_repository::pg::Repository::new(pool.clone()));

        let sync = Arc::new(user_services::Sync::new(repository));

        Self {
            sync,
        }
    }
}

#[derive(Debug)]
pub struct StorageServices{
    pub create: Arc<storage_services::Create>,
    pub list: Arc<storage_services::List>,
}

impl StorageServices {
    fn new(pool: &PgPool) -> Self {
        let repository1 = Box::new(storage_repository::pg::Repository::new(pool.clone()));
        let repository2 = Box::new(storage_repository::pg::Repository::new(pool.clone()));

        let create = Arc::new(storage_services::Create::new(repository1));
        let list = Arc::new(storage_services::List::new(repository2));

        Self {
            create,
            list,
        }
    }
}

pub type AppState = web::Data<AppStateInner>;

async fn run_http_server(
    host: String,
    port: u16,
    jwt_pub_key: String,
    state: AppStateInner,
) -> std::io::Result<()> {
    let key = Box::new(DecodingKey::from_rsa_pem(jwt_pub_key.as_bytes()).map_err(to_std_io_err)?);

    let state = AppState::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(TracingLogger::default())
            .configure(configure_api(*(key.clone())))
            .service(endpoints::telemetry::healthz)
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
        cfg
            .service(
                web::scope("/api")
                    // .wrap(keycloak_auth)
                    .service(
                        web::scope("/users")
                            .service(endpoints::users::sync)
                    )
                    .service(
                        web::scope("/storages")
                            .service(endpoints::storages::create)
                            .service(endpoints::storages::list)
                    )
            );
    })
}