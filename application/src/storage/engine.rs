pub mod fs;
use async_trait::async_trait;
use thiserror::Error;
use tokio::io::{self, AsyncRead};

/// Represents the storage engine.
/// Engine contains low-level implementation of file manipulation.
#[async_trait]
pub(crate) trait Engine {
    type Path;

    /// Puts file to storage.
    async fn put<R>(&self, name: &str, source: &mut R) -> PutResult<Self::Path>
    where
        R: AsyncRead + Unpin + Send;

    /// Deletes file from storage.
    async fn delete(&self, path: Self::Path) -> DeleteResult;
}

pub(crate) type PutResult<Path> = Result<Path, PutError>;

#[derive(Debug, Error)]
pub(crate) enum PutError {
    #[error("IO failed")]
    IO(#[from] io::Error),
}

pub(crate) type DeleteResult = Result<(), DeleteError>;

#[derive(Debug, Error)]
pub(crate) enum DeleteError {
    #[error("IO failed")]
    IO(#[from] io::Error),
}

pub(crate) trait Factory {
    type Engine: Engine;

    fn create(&self) -> Result<Self::Engine, anyhow::Error>;
}
