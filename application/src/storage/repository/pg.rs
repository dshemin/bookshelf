use async_trait::async_trait;
use sqlx::{QueryBuilder, types, query, Row};
use sqlx::postgres::{PgPool, PgRow};
use crate::storage::{self, Storage, Settings};
use crate::{PaginatedData, LIMIT};
use futures::{future, TryStreamExt};

#[derive(Clone, Debug)]
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

impl Repository {
    fn hydrate(row: PgRow) -> Storage {
        let settings: types::Json<Settings> = row.get("settings");

        Storage {
            id: row.get("id"),
            name: row.get("name"),
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
            .execute(&self.pool).await?;

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
            .fetch_optional(&self.pool).await?;

        let row = row.map(Self::hydrate);
        Ok(row)
    }

    async fn delete(&self, id: storage::ID) -> anyhow::Result<()> {
        query(r#"DELETE FROM "storages" WHERE ID = $1"#)
            .bind(id)
            .execute(&self.pool).await?;

        Ok(())
    }
}
