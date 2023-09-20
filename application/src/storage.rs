mod engine;
mod entity;

pub mod repository;
pub mod service;

pub use entity::*;
use std::fmt::Debug;

use async_trait::async_trait;

use crate::PaginatedData;

/// The storage repository.
/// Represents all necessary staff for managing storages.
#[async_trait]
pub trait Repository: Debug {
    /// Add new storage with provided data.
    async fn insert(&self, dto: &InsertDTO) -> anyhow::Result<()>;

    /// Fetch list of all storages from specified id but not including it.
    async fn list(&self, from: Option<ID>) -> anyhow::Result<PaginatedData<Storage>>;

    /// Fetch the storage by id.
    async fn get(&self, id: ID) -> anyhow::Result<Option<Storage>>;

    /// Update provided storage.
    async fn update(&self, id: ID, dto: &UpdateDTO) -> anyhow::Result<Option<Storage>>;

    /// Delete provided storage.
    async fn delete(&self, id: ID) -> anyhow::Result<()>;
}

/// Represents necessary information for creating new storage.
pub struct InsertDTO {
    pub id: ID,
    pub name: String,
    pub settings: Settings,
}

/// Represents necessary information for updating new storage.
pub struct UpdateDTO {
    pub name: String,
    pub settings: Settings,
}
