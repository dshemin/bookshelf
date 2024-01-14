use crate::storage::{self, Settings, Storage};
use crate::{PaginatedData, LIMIT};
use async_trait::async_trait;
use futures::{future, TryStreamExt};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{query, types, QueryBuilder, Row};

/// Implementation of storage's repository for PostgreSQL.
#[derive(Clone, Debug)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    /// Create new PostgreSQL storage's repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Repository {
    /// Transform PostgreSQL's row to storage entity.
    fn hydrate(row: PgRow) -> Storage {
        let settings: types::Json<Settings> = row.get("settings");

        let name: String = row.get("name");

        Storage {
            id: row.get("id"),
            name: storage::Name::new_valid(name),
            settings: settings.0,
        }
    }
}

#[async_trait]
impl storage::Repository for Repository {
    async fn insert(&self, dto: &storage::InsertDTO) -> anyhow::Result<()> {
        query(r#"INSERT INTO "storages" (id, name, settings) VALUES ($1, $2, $3)"#)
            .bind(dto.id)
            .bind(&dto.name)
            .bind(types::Json(&dto.settings))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn list(&self, from: Option<storage::ID>) -> anyhow::Result<PaginatedData<Storage>> {
        let mut qb = QueryBuilder::new(r#"SELECT * FROM "storages""#);

        if let Some(id) = from {
            qb.push("WHERE id > ");
            qb.push_bind(id);
        }

        qb.push(format!(r#" ORDER BY "id" LIMIT {}"#, LIMIT));

        let stream = qb.build().fetch(&self.pool);

        let mut storages = Vec::new();

        stream
            .try_for_each(|row: PgRow| {
                let s = Self::hydrate(row);

                storages.push(s);
                future::ready(Ok(()))
            })
            .await?;

        let mut res = PaginatedData {
            data: storages,
            cursor: None,
        };

        if res.data.len() == LIMIT {
            if let Some(v) = res.data.last() {
                res.cursor = Some(crate::Cursor::new(Some(v.id)));
            }
        }

        Ok(res)
    }

    async fn get(&self, id: storage::ID) -> anyhow::Result<Option<Storage>> {
        let row = query(r#"SELECT * FROM "storages" WHERE ID = $1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let row = row.map(Self::hydrate);
        Ok(row)
    }

    async fn update(
        &self,
        id: storage::ID,
        dto: &storage::UpdateDTO,
    ) -> anyhow::Result<Option<Storage>> {
        let row = query(
            r#"
            UPDATE "storages"
            SET
                name = $1,
                settings = $2
            WHERE ID = $3
            RETURNING
                id,
                name,
                settings
        "#,
        )
        .bind(&dto.name)
        .bind(types::Json(&dto.settings))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        if row.is_empty() {
            return Ok(None);
        }

        Ok(Some(Self::hydrate(row)))
    }

    async fn delete(&self, id: storage::ID) -> anyhow::Result<()> {
        query(r#"DELETE FROM "storages" WHERE ID = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
