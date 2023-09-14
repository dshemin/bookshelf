pub(super) mod fs;

use super::engine::fs::Engine as FSEngine;
use super::entity::Path;
use tokio::io::AsyncRead;

/// The engine.
/// An enum for all available engines.
pub enum Engine {
    FS(FSEngine),
}

impl Engine {
    /// Puts a data to the storage.
    pub async fn put<R>(&self, name: &str, source: &mut R) -> Result<Path, anyhow::Error>
    where
        R: AsyncRead + Unpin + Send,
    {
        let path = match self {
            Self::FS(engine) => Path::FS(engine.put(name, source).await?),
        };

        Ok(path)
    }

    /// Deletes a file under the path.
    pub async fn delete(&self, path: String) -> Result<(), anyhow::Error> {
        match self {
            Self::FS(engine) => engine.delete(path).await,
        }
    }
}
