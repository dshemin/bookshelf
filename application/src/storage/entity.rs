use std::path::PathBuf;

use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain_type;

use super::engine::fs::Engine as FSEngine;
use super::engine::Engine;

/// The storage.
///
/// Holds all information about connected storage and provide a method to connect
/// to it.
#[derive(Serialize)]
pub struct Storage {
    pub(crate) id: ID,
    pub(crate) name: Name,
    pub(crate) settings: Settings,
}

impl Storage {
    /// Creates new storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use application::storage::{Name, Storage, Settings};
    ///
    /// let settings = Settings::FS{
    ///     base_path: "/tmp/foo".to_owned(),
    /// };
    ///
    /// let storage = Storage::new(Name::new("foo").unwrap(), settings);
    /// ```
    pub fn new(name: Name, settings: Settings) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
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

/// Storage unique identifier.
pub type ID = Uuid;

domain_type!(Name, NameResult, String, length(min = 3, max = 255));

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

#[cfg(test)]
mod tests {
    use super::*;

    mod name {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn valid() {
                let result = Name::new("foo");

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), Name("foo".to_owned()));
            }

            #[test]
            fn invalid_less_than_min() {
                let result = Name::new("fo");

                assert!(result.is_err());
                let err = result.err().unwrap();
                assert_eq!("length is lower than 3", err.to_string());
            }

            #[test]
            fn invalid_greater_than_max() {
                let result = Name::new("1".repeat(256));

                assert!(result.is_err());
                let err = result.err().unwrap();
                assert_eq!("length is greater than 255", err.to_string());
            }
        }
    }
}
