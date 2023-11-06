use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::engine::fs::Engine as FSEngine;
use super::engine::Engine;

/// Storage unique identifier.
pub type ID = Uuid;

/// The storage.
///
/// Holds all information about connected storage and provide a method to connect
/// to it.
#[derive(Serialize)]
pub struct Storage {
    pub(crate) id: ID,
    pub(crate) name: String,
    pub(crate) settings: Settings,
}

impl Storage {
    /// Creates new storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::storage::{Storage, Settings};
    ///
    /// let settings = Settings::FS{
    ///     base_path: "/tmp/foo".to_owned(),
    /// };
    ///
    /// let engine = Storage::new("foo", settings);
    /// ```
    pub fn new<T>(name: T, settings: Settings) -> Self
    where
        T: Into<String>,
    {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            settings,
        }
    }

    /// Connects this storage to physical storage.
    //
    /// Will return an engine which can be used for managing data on the physical
    /// storage.
    pub async fn connect(&self) -> Result<Engine, anyhow::Error> {
        let engine = match &self.settings {
            Settings::FS { base_path } => Engine::FS(FSEngine::new(base_path)?),
        };

        Ok(engine)
    }
}

/// Settings for the storage.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Settings {
    /// Settings for filesystem storage.
    #[serde(rename = "fs")]
    FS {
        /// Path to the directory where all uploaded books will be stored.
        base_path: String,
    },
}

/// The path to file on specific storage.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Path {
    #[serde(rename = "fs")]
    FS(PathBuf),
}
