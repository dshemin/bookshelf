use async_trait::async_trait;
use sqlx::{QueryBuilder, types, query, Row};
use sqlx::postgres::{PgPool, PgRow};
use crate::storage::{self, Storage, Settings};
use crate::{PaginatedData, LIMIT};
use futures::{future, TryStreamExt};

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
impl storage::Repository for Repository {
    async fn insert(&self, dto: &storage::InsertDTO) -> anyhow::Result<()> {
        query(r#"INSERT INTO "storages" (id, name, settings) VALUES ($1, $2, $3)"#)
            .bind(dto.id)
            .bind(&dto.name)
            .bind(types::Json(&dto.settings))
            .execute(&self.pool).await?;

        Ok(())
    }

    async fn get(&self, from: Option<storage::ID>) -> anyhow::Result<PaginatedData<Storage>> {
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
                let settings: types::Json<Settings> = row.get("settings");

                let s = Storage {
                    id: row.get("id"),
                    name: row.get("name"),
                    settings: settings.0,
                };

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
}
