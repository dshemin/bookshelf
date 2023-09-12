use derive_new::new;

use crate::{storage, Cursor, PaginatedData};

use super::Storage;

/// Create service.
///
/// Creates new storage.
#[derive(Debug, new)]
pub struct Create {
    storages_repository: Repository,
}

impl Create {
    pub async fn create(&self, name: String, settings: storage::Settings) -> anyhow::Result<()> {
        let dto = storage::InsertDTO {
            id: storage::ID::new_v4(),
            name,
            settings,
        };
        self.storages_repository.insert(&dto).await
    }
}

#[derive(Debug, new)]
pub struct List {
    storages_repository: Repository,
}

impl List {
    pub async fn list(&self, cursor: Option<Cursor>) -> anyhow::Result<PaginatedData<Storage>> {
        self.storages_repository.list(cursor.and_then(|x| { x.last_id })).await
    }
}

#[derive(Debug, new)]
pub struct Get {
    storages_repository: Repository,
}

impl Get {
    pub async fn get(&self, id: storage::ID) -> anyhow::Result<Option<Storage>> {
        self.storages_repository.get(id).await
    }
}

#[derive(Debug, new)]
pub struct Delete {
    storages_repository: Repository,
}

impl Delete {
    pub async fn delete(&self, id: storage::ID) -> anyhow::Result<()> {
        self.storages_repository.delete(id).await
    }
}

type Repository = Box<dyn storage::Repository + Send + std::marker::Sync>;
