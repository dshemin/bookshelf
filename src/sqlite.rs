use diesel::Connection as CC;
use diesel::sqlite::SqliteConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use log::{Level, debug, info, log_enabled};

use anyhow::anyhow;

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
