mod engine;
mod entity;

pub mod repository;
pub mod service;

use std::fmt::Debug;
pub use entity::*;

use async_trait::async_trait;

use crate::PaginatedData;

#[async_trait]
pub trait Repository: Debug {
    async fn insert(&self, u: &InsertDTO) -> anyhow::Result<()>;

    async fn list(&self, from: Option<ID>) -> anyhow::Result<PaginatedData<Storage>>;

    async fn get(&self, id: ID) -> anyhow::Result<Option<Storage>>;
}

pub struct InsertDTO {
    pub id: ID,
    pub name: String,
    pub settings: Settings,
}
