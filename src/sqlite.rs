use diesel::backend::Backend;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Binary;
use diesel::sqlite::{Sqlite, SqliteConnection};
use diesel::{Connection as CC, deserialize, serialize};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use log::{Level, debug, info, log_enabled};

use anyhow::anyhow;

use uuid::Uuid;

pub type Connection = SyncConnectionWrapper<SqliteConnection>;
pub type ConnectionPool = Pool<Connection>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub async fn connect(db_url: &str) -> anyhow::Result<ConnectionPool> {
    migrate(db_url)?;
    create_pool(db_url)
}

fn migrate(db_url: &str) -> anyhow::Result<()> {
    info!("migrate DB");

    // To simplify code, just create ordinal sync connection without pool and
    // apply all migrations.
    let mut conn = Box::new(diesel::SqliteConnection::establish(db_url)?);

    if log_enabled!(Level::Debug) {
        conn.pending_migrations(MIGRATIONS)
            .map_err(|err| anyhow!(err))?
            .iter()
            .for_each(|m| {
                debug!("apply migration {}", m.name().to_string());
            });
    }

    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|err| anyhow!(err))?;

    Ok(())
}

fn create_pool(db_url: &str) -> anyhow::Result<ConnectionPool> {
    let manager = AsyncDieselConnectionManager::<Connection>::new(db_url);
    Pool::builder(manager).build().map_err(|err| anyhow!(err))
}

#[derive(Debug, AsExpression, FromSqlRow, Copy, Clone)]
#[diesel(sql_type = Binary)]
pub struct ID(pub Uuid);

impl ID {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl ToSql<Binary, Sqlite> for ID {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let bytes = self.0.as_bytes();
        <[u8] as ToSql<Binary, Sqlite>>::to_sql(bytes, out)
    }
}

impl FromSql<Binary, Sqlite> for ID {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let bytes: Vec<u8> = <Vec<u8> as FromSql<Binary, Sqlite>>::from_sql(bytes)?;
        Ok(ID(Uuid::from_bytes(
            bytes
                .try_into()
                .map_err(|v| format!("invalid UUID {:?}", v))?,
        )))
    }
}
