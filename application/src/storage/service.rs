use derive_new::new;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::AsyncRead;

use crate::{storage, Cursor, PaginatedData};

use super::{Storage, Path};

/// Creator service.
///
/// Creates new storage.
#[derive(Debug, new)]
pub struct Creator {
    storages_repository: Repository,
}

impl Creator {
    /// Create new storage.
    pub async fn create(
        &self,
        name: String,
        settings: storage::Settings,
    ) -> anyhow::Result<storage::ID> {
        let dto = storage::InsertDTO {
            id: storage::ID::new_v4(),
            name,
            settings,
        };
        self.storages_repository.insert(&dto).await?;
        Ok(dto.id)
    }
}

/// Lister service.
///
/// Lists storages.
#[derive(Debug, new)]
pub struct Lister {
    storages_repository: Repository,
}

impl Lister {
    /// List storages.
    pub async fn list(&self, cursor: Option<Cursor>) -> anyhow::Result<PaginatedData<Storage>> {
        self.storages_repository
            .list(cursor.and_then(|x| x.last_id))
            .await
    }
}

/// Getter service.
///
/// Get single storage.
#[derive(Debug, new)]
pub struct Getter {
    storages_repository: Repository,
}

impl Getter {
    /// Get storage.
    pub async fn get(&self, id: storage::ID) -> anyhow::Result<Option<Storage>> {
        self.storages_repository.get(id).await
    }
}

/// Updater service.
///
/// Updates single storage.
#[derive(Debug, new)]
pub struct Updater {
    storages_repository: Repository,
}

impl Updater {
    /// Update storage.
    pub async fn update(
        &self,
        id: storage::ID,
        name: String,
        settings: storage::Settings,
    ) -> UpdateResult {
        let s = self
            .storages_repository
            .update(id, &storage::UpdateDTO { name, settings })
            .await
            .map_err(|e| UpdateError::DB(e.to_string()))?;

        s.ok_or(UpdateError::NotFound())
    }
}

pub type UpdateResult = Result<Storage, UpdateError>;

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("repository error: {0}")]
    DB(String),

    #[error("not found")]
    NotFound(),
}

/// Deleter service.
///
/// Deletes single storage.
#[derive(Debug, new)]
pub struct Deleter {
    storages_repository: Repository,
}

impl Deleter {
    /// Delete storage.
    pub async fn delete(&self, id: storage::ID) -> anyhow::Result<()> {
        self.storages_repository.delete(id).await
    }
}

/// FileUploader service.
///
/// Uploads file to given storage.
#[derive(Debug, new)]
pub struct FileUploader {
    getter: Arc<Getter>,
}

impl FileUploader {
    /// Delete storage.
    pub async fn upload<R>(
        &self,
        id: storage::ID,
        name: &str,
        source: &mut R,
    ) -> FileUploadResult
    where
        R: AsyncRead + Unpin + Send,
    {
        let storage = self.getter
            .get(id)
            .await
            .map_err(FileUploadError::GetStorage)?
            .ok_or(FileUploadError::StorageNotFound)?;

        let engine = storage.connect().await.map_err(FileUploadError::ConnectStorage)?;

        let path = engine.put(name, source).await.map_err(FileUploadError::PutFileToStorage)?;

        Ok(path)
    }
}

pub type FileUploadResult = Result<Path, FileUploadError>;

#[derive(Debug, Error)]
pub enum FileUploadError {
    #[error("get storage: {0}")]
    GetStorage(#[source] anyhow::Error),

    #[error("storage not found")]
    StorageNotFound,

    #[error("connect storage")]
    ConnectStorage(#[source] anyhow::Error),

    #[error("put file to storage")]
    PutFileToStorage(#[source] anyhow::Error),
}

type Repository = Box<dyn storage::Repository + Send + std::marker::Sync>;
