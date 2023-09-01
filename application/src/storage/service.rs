use derive_new::new;

use crate::{storage, Cursor, PaginatedData};

use super::Storage;

/// Create service.
///
/// Creates new storage.
#[derive(new)]
pub struct Create {
    storages_repository: Box<dyn storage::Repository + Send + std::marker::Sync>,
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

impl std::fmt::Debug for Create {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(new)]
pub struct List {
    storages_repository: Box<dyn storage::Repository + Send + std::marker::Sync>,
}

impl List {
    pub async fn list(&self, cursor: Option<Cursor>) -> anyhow::Result<PaginatedData<Storage>> {
        self.storages_repository.get(cursor.and_then(|x| { x.last_id })).await
    }
}

impl std::fmt::Debug for List {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
