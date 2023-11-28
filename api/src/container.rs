use application::storage::{repository as storage_repository, service as storage_services};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use crate::config::Config;

#[derive(Clone)]
pub struct Container {
    pub storage_create: Arc<storage_services::Creator>,
    pub storage_list: Arc<storage_services::Lister>,
    pub storage_get: Arc<storage_services::Getter>,
    pub storage_update: Arc<storage_services::Updater>,
    pub storage_delete: Arc<storage_services::Deleter>,
    pub storage_file_uploader: Arc<storage_services::FileUploader>,
}

impl Container {
    pub async fn new(cfg: &Config) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new().connect(&cfg.pg.conn_uri).await?;

        let repository = Box::new(storage_repository::pg::Repository::new(pool));

        let storage_get = Arc::new(storage_services::Getter::new(repository.clone()));

        Ok(Self {
            storage_create: Arc::new(storage_services::Creator::new(repository.clone())),
            storage_list: Arc::new(storage_services::Lister::new(repository.clone())),
            storage_get: storage_get.clone(),
            storage_update: Arc::new(storage_services::Updater::new(repository.clone())),
            storage_delete: Arc::new(storage_services::Deleter::new(repository)),
            storage_file_uploader: Arc::new(storage_services::FileUploader::new(storage_get)),
        })
    }
}
