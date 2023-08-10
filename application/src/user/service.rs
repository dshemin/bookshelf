use crate::user::{self, User};

/// Sync service.
///
/// Synchronize user from remote source to the application.
pub struct Sync {
    users_repository: Box<dyn user::Repository + Send + std::marker::Sync>,
}

impl Sync {
    pub fn new(users_repository: Box<dyn user::Repository + Send + std::marker::Sync>) -> Self {
        Self {
            users_repository,
        }
    }

    pub async fn sync(&self, id: user::ID) -> anyhow::Result<()> {
        self.users_repository.upsert(&User::new(id, user::Role::Ordinary)).await?;
        Ok(())
    }
}

impl std::fmt::Debug for Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")?;
        Ok(())
    }
}
