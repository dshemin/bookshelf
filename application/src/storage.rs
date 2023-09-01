mod engine;
mod entity;

pub mod repository;
pub mod service;

pub use entity::*;

use async_trait::async_trait;

use crate::PaginatedData;

#[async_trait]
pub trait Repository {
    async fn insert(&self, u: &InsertDTO) -> anyhow::Result<()>;

    async fn get(&self, from: Option<ID>) -> anyhow::Result<PaginatedData<Storage>>;
}

pub struct InsertDTO {
    pub id: ID,
    pub name: String,
    pub settings: Settings,
}
