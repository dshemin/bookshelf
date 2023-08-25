use async_trait::async_trait;
use sqlx::postgres::PgPool;
use sqlx::types;
use sqlx::query;
use crate::storage;

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
impl storage::Repository for Repository {
    async fn insert(&self, dto: &storage::InsertDTO) -> anyhow::Result<()> {
        query(r#"INSERT INTO "storages" (id, name, settings) VALUES ($1, $2, $3)"#)
            .bind(dto.id)
            .bind(&dto.name)
            .bind(types::Json(serde_json::to_string(&dto.settings)?))
            .execute(&self.pool).await?;

        Ok(())
    }
}
