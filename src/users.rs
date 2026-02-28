use crate::{
    schema::users::{self, dsl},
    sqlite::{self, ID},
};
use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::{
    insert_into,
    prelude::*,
    result::{DatabaseErrorKind, Error as DatabaseError},
};
use diesel_async::{pooled_connection::deadpool::PoolError, RunQueryDsl};
use std::sync::LazyLock;
use thiserror::Error;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: ID,
    pub login: String,
    pub password: String,
    pub role: String,
}

impl User {
    pub fn new(login: String, password: String, role: String) -> anyhow::Result<Self> {
        Ok(User {
            id: ID::new(),
            login: login,
            password: Self::hash_password(&password)?,
            role: role,
        })
    }

    fn hash_password(password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(ARGON
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| anyhow::anyhow!("{}", err))?
            .to_string())
    }
}

static ARGON: LazyLock<Argon2> = LazyLock::new(Argon2::default);

pub type UserCollection = Vec<User>;

#[derive(Clone)]
pub struct Service {
    pool: sqlite::ConnectionPool,
}

impl Service {
    pub fn new(pool: sqlite::ConnectionPool) -> Self {
        Self { pool: pool }
    }

    pub async fn create(&self, user: User) -> Result<(), CreateError> {
        let mut conn = self.pool.get().await?;

        insert_into(dsl::users)
            .values(&user)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn list(&self) -> anyhow::Result<UserCollection> {
        let mut conn = self.pool.get().await?;

        users::table
            .load::<User>(&mut conn)
            .await
            .map_err(|err| anyhow!(err))
    }
}

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("user already exists")]
    AlreadyExists,

    #[error("{0}")]
    PoolError(#[from] PoolError),

    #[error("{0}")]
    ResultError(diesel::result::Error),
}

impl From<DatabaseError> for CreateError {
    fn from(value: DatabaseError) -> Self {
        match value {
            DatabaseError::DatabaseError(kind, info)
                if kind == DatabaseErrorKind::UniqueViolation =>
            {
                CreateError::AlreadyExists
            }
            err => CreateError::ResultError(err),
        }
    }
}
