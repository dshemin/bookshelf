pub mod service;
pub mod repository;

use async_trait::async_trait;
use parse_display::{Display, FromStr};

pub type ID = uuid::Uuid;

#[derive(Debug)]
pub struct User {
    id: ID,
    role: Role,
}

#[derive(Debug, Display, FromStr)]
#[display(style = "lowercase")]
pub enum Role {
    Admin,
    Ordinary,
}

impl User {
    pub fn new(id: ID, role: Role) -> Self {
        Self {
            id,
            role,
        }
    }

    pub fn id(&self) -> &ID { &self.id }
    pub fn role(&self) -> &Role { &self.role }
}

#[async_trait]
pub trait Repository {
    async fn upsert(&self, u: &User) -> anyhow::Result<()>;

    async fn find_by_id(&self, id: ID) -> anyhow::Result<User>;
}
