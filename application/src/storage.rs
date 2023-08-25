mod engine;
mod entity;
pub mod repository;
pub mod service;

pub use entity::*;

use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    async fn insert(&self, u: &InsertDTO) -> anyhow::Result<()>;
}

pub struct InsertDTO {
    pub id: ID,
    pub name: String,
    pub settings: Settings,
}
