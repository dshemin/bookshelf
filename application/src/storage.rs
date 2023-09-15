mod engine;
mod entity;

pub mod repository;
pub mod service;

pub use entity::*;
use std::fmt::Debug;

use async_trait::async_trait;

use crate::PaginatedData;

#[async_trait]
pub trait Repository: Debug {
    async fn insert(&self, dto: &InsertDTO) -> anyhow::Result<()>;

    async fn list(&self, from: Option<ID>) -> anyhow::Result<PaginatedData<Storage>>;

    async fn get(&self, id: ID) -> anyhow::Result<Option<Storage>>;

    async fn update(&self, id: ID, dto: &UpdateDTO) -> anyhow::Result<Option<Storage>>;

    async fn delete(&self, id: ID) -> anyhow::Result<()>;
}

pub struct InsertDTO {
    pub id: ID,
    pub name: String,
    pub settings: Settings,
}

pub struct UpdateDTO {
    pub name: String,
    pub settings: Settings,
}
