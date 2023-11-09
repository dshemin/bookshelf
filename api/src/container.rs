use application::storage::{repository as storage_repository, service as storage_services};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use crate::config::Config;

#[derive(Clone)]
pub struct Container {
    pub storage_create: Arc<storage_services::Create>,
    pub storage_list: Arc<storage_services::List>,
    pub storage_get: Arc<storage_services::Get>,
    pub storage_update: Arc<storage_services::Update>,
    pub storage_delete: Arc<storage_services::Delete>,
}

impl Container {
    pub async fn new(cfg: &Config) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new().connect(&cfg.pg.conn_uri).await?;

        let repository = Box::new(storage_repository::pg::Repository::new(pool));

        Ok(Self {
            storage_create: Arc::new(storage_services::Create::new(repository.clone())),
            storage_list: Arc::new(storage_services::List::new(repository.clone())),
            storage_get: Arc::new(storage_services::Get::new(repository.clone())),
            storage_update: Arc::new(storage_services::Update::new(repository.clone())),
            storage_delete: Arc::new(storage_services::Delete::new(repository)),
        })
    }
}
