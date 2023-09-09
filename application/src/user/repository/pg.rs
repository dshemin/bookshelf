use async_trait::async_trait;
use sqlx::postgres::PgPool;
use sqlx::{query, query_as};
use crate::user::{self, User};

#[derive(Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
        }
    }
}

#[async_trait]
impl user::Repository for Repository {
    async fn upsert(&self, u: &User) -> anyhow::Result<()> {
        query(r#"INSERT INTO "users" (id, role) VALUES ($1, $2)"#)
            .bind(u.id())
            .bind(u.role().to_string())
            .execute(&self.pool).await?;

        Ok(())
    }

    async fn find_by_id(&self, id: user::ID) -> anyhow::Result<User> {
        let u = query_as::<_, UserDB>(
            r#"SELECT FROM "users" WHERE id = $1"#
        )
            .bind(id)
            .fetch_one(&self.pool).await?;

        Ok(u.try_into()?)
    }
}

#[derive(sqlx::FromRow)]
struct UserDB {
    id: user::ID,
    role: String,
}

impl TryInto<User> for UserDB {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<User, Self::Error> {
        Ok(User {
            id: self.id,
            role: self.role.parse()?,
        })
    }
}

impl From<User> for UserDB {
    fn from(value: User) -> Self {
        Self {
            id: value.id().to_owned(),
            role: value.role().to_string(),
        }
    }
}
