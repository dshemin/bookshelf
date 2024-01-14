use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use super::engine::fs::Engine as FSEngine;
use super::engine::Engine;

/// Storage unique identifier.
pub type ID = Uuid;

/// Storage name.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Name(String);

const NAME_MIN_LEN: usize = 3;
const NAME_MAX_LEN: usize = 255;

impl Name {
    pub fn new<T>(value: T) -> NameResult
    where
        T: Into<String>,
    {
        let v: String = value.into();

        if let Some(err) = Self::validate(&v) {
            return Err(err);
        }
        Ok(Self(v))
    }

    pub(crate) fn new_valid<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self(value.into())
    }

    fn validate(value: &str) -> Option<NameError> {
        if value.len() < NAME_MIN_LEN {
            return Some(NameError::TooShort);
        }

        if value.len() > NAME_MAX_LEN {
            return Some(NameError::TooLong);
        }

        None
    }
}

impl Into<String> for Name {
    fn into(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Name {
    type Error = NameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Name::new(value)
    }
}

impl TryFrom<&str> for Name {
    type Error = NameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Name::new(value)
    }
}

pub type NameResult = Result<Name, NameError>;

#[derive(Debug, Error)]
pub enum NameError {
    #[error("too short, should have at least {NAME_MIN_LEN} characters")]
    TooShort,

    #[error("too long, should have no more than {NAME_MAX_LEN} characters")]
    TooLong,
}

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
    /// let engine = Storage::new(Name::new("foo").unwrap(), settings);
    /// ```
    pub fn new(name: Name, settings: Settings) -> NewResult {
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            settings,
        })
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

pub type NewResult = Result<Storage, NewError>;

#[derive(Debug, Error)]
pub enum NewError {
    #[error("invalid name: {0}")]
    InvalidName(#[from] NameError),
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
                assert!(matches!(result.err().unwrap(), NameError::TooShort));
            }

            #[test]
            fn invalid_greater_than_max() {
                let result = Name::new("1".repeat(NAME_MAX_LEN + 1));

                assert!(result.is_err());
                assert!(matches!(result.err().unwrap(), NameError::TooLong));
            }
        }
    }
}
