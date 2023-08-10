use std::path::PathBuf;

use uuid::Uuid;
use serde::{Serialize, Deserialize};

use super::engine::Engine;
use super::engine::fs::Engine as FSEngine;

/// Storage unique identifier.
pub type ID = Uuid;

/// The storage.
///
/// Holds all information about connected storage and provide a method to connect
/// to it.
pub struct Storage {
    id: ID,
    name: String,
    settings: Settings,
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

    /// Returns an unique identificator of this storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::storage::{Storage, Settings};
    //
    /// let settings = Settings::FS{
    ///     base_path: "/tmp/foo".to_owned(),
    /// };
    ///
    /// let engine = Storage::new("foo", settings);
    ///
    /// assert_eq!("foo", engine.name());
    /// ```
    pub fn id(&self) -> &ID {
        &self.id
    }

    /// Returns an unique identificator of this storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::storage::{Storage, Settings};
    //
    /// let settings = Settings::FS{
    ///     base_path: "/tmp/foo".to_owned(),
    /// };
    ///
    /// let engine = Storage::new("foo", settings);
    ///
    /// assert_eq!("foo", engine.name());
    /// ```
    pub fn name(&self) -> &str {
        &self.name
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
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Settings {
    /// Settings for filesystem storage.
    FS {
        /// Path to the directory where all uploaded books will be stored.
        base_path: String,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Path {
    FS(PathBuf),
}

pub type PutResult = Result<Path, anyhow::Error>;
