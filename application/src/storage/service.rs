use crate::storage;

/// Sync service.
///
/// Synchronize user from remote source to the application.
pub struct Create {
    storages_repository: Box<dyn storage::Repository + Send + std::marker::Sync>,
}

impl Create {
    pub fn new(storages_repository: Box<dyn storage::Repository + Send + std::marker::Sync>) -> Self {
        Self {
            storages_repository,
        }
    }

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
