use async_trait::async_trait;
use uuid::Uuid;

use super::entity::Storage;

#[async_trait]
pub trait Repository {
    async fn get(&self, id: &Uuid) -> Result<Storage, anyhow::Error>;
}
