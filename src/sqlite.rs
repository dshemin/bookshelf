use diesel::sqlite::SqliteConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;

use anyhow::anyhow;

pub type Connection = SyncConnectionWrapper<SqliteConnection>;
pub type ConnectionPool = Pool<Connection>;

pub async fn connect(db_url: &str) -> anyhow::Result<ConnectionPool> {
    let manager = AsyncDieselConnectionManager::<Connection>::new(db_url);

    Pool::builder(manager).build().map_err(|err| anyhow!(err))
}
