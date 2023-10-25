use std::path::PathBuf;

use thiserror::Error;
use tokio::fs::{remove_file, try_exists, File};
use tokio::io::{self, AsyncRead};

/// Represents file system engine.
///
/// For now it's a simple engine which will put all files inside provided
/// directory.
/// It not optimal but deadly simple and will be more then enough for small
/// amount of books.
#[derive(Debug)]
pub struct Engine {
    /// Base directory where all files will be placed.
    base_path: PathBuf,
}

impl Engine {
    /// Creates new FS engine with provided base path.
    ///
    /// # Errors
    ///
    /// Will return an error if provided base path pointed to a file but not a
    /// directory, or can't create the base directory.
    pub fn new<T>(base_path: T) -> Result<Self, FSNewError>
    where
        T: Into<PathBuf>,
    {
        let path = base_path.into();

        Self::make_base_dir(&path)?;

        Ok(Self { base_path: path })
    }

    /// Make base directory.
    fn make_base_dir(path: &PathBuf) -> Result<(), FSNewError> {
        if !path.exists() {
            std::fs::create_dir_all(path)?;
            return Ok(());
        }

        if !path.is_dir() {
            return Err(FSNewError::BasePathNotDir);
        }

        Ok(())
    }

    /// Puts a bytes from given async reader to the storage with given name.
    ///
    /// # Errors
    ///
    /// Will return an error if failed read from given source or write to
    /// the file.
    pub async fn put<R>(&self, name: &str, source: &mut R) -> Result<PathBuf, anyhow::Error>
    where
        R: AsyncRead + Unpin + Send,
    {
        let path = {
            let mut p = self.base_path.clone();
            p.push(name);
            p
        };

        let mut dest = File::create(&path).await?;

        io::copy(source, &mut dest).await?;

        Ok(path)
    }

    /// Deletes a file under the given path from the storage.
    ///
    /// # Errors
    ///
    /// Will return an error if failed to delete required file.
    pub async fn delete(&self, path: &PathBuf) -> Result<(), anyhow::Error> {
        if try_exists(path).await? {
            remove_file(path).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum FSNewError {
    #[error("base path not a directory")]
    BasePathNotDir,

    #[error("failed to create directory under base path")]
    FailedToCreateBaseDir(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    use scopeguard::defer;

    mod engine {
        use std::pin::Pin;
        use std::task::{Context, Poll};

        use tokio::io::ReadBuf;

        use super::*;

        mod new {
            use super::*;
            use std::fs::{create_dir_all, write};

            #[test]
            fn base_path_not_exists() {
                let path = &get_temp_path("not_exists_dir");
                defer! {
                    safe_remove(path).unwrap();
                };

                let result = Engine::new(path);

                assert!(result.is_ok());
                assert!(path.exists());
            }

            #[test]
            fn base_path_exists_and_it_is_a_directory() {
                let path = &get_temp_path("al");
                create_dir_all(path).unwrap();
                defer! {
                    safe_remove(path).unwrap();
                };

                let result = Engine::new(path);

                assert!(result.is_ok());
                assert!(path.exists());
            }

            #[test]
            fn base_path_exists_and_it_is_a_file() {
                let path = &get_temp_path("a_file");
                write(path, "a test").unwrap();
                defer! {
                    safe_remove(path).unwrap();
                };

                let result = Engine::new(path);

                assert!(result.is_err());
                assert!(matches!(result.err().unwrap(), FSNewError::BasePathNotDir));
                assert!(path.exists());
            }
        }

        mod put {
            use super::*;
            use tokio::fs::read;

            #[tokio::test]
            async fn success() {
                const NAME: &str = "file";
                let base_path = &get_temp_path("put_success");
                defer! {
                    safe_remove(base_path).unwrap();
                };

                let engine = Engine::new(base_path).unwrap();

                let mut source = StringAsyncReader::new("some data");

                let actual_path = engine.put("file", &mut source).await.unwrap();

                let expected_path = base_path.join(NAME);

                assert!(base_path.exists());
                assert_eq!("some data".as_bytes(), read(&expected_path).await.unwrap());
                assert_eq!(expected_path, actual_path);
            }
        }

        mod delete {
            use super::*;
            use tokio::fs::write;

            #[tokio::test]
            async fn exists() {
                const NAME: &str = "file";
                let base_path = &get_temp_path("delete_exists");
                defer! {
                    safe_remove(base_path).unwrap();
                };

                let engine = Engine::new(base_path).unwrap();

                let path = base_path.join(NAME);
                write(&path, "a test").await.unwrap();

                assert!(&path.exists());

                engine
                    .delete(&path)
                    .await
                    .unwrap();

                assert!(!&path.exists());
            }

            #[tokio::test]
            async fn not_exists() {
                const NAME: &str = "file";
                let base_path = &get_temp_path("delete_not_exists");
                defer! {
                    safe_remove(base_path).unwrap();
                };

                let engine = Engine::new(base_path).unwrap();

                let path = base_path.join(NAME);

                assert!(!&path.exists());

                engine
                    .delete(&path)
                    .await
                    .unwrap();

                assert!(!&path.exists());
            }
        }

        struct StringAsyncReader {
            once: std::sync::Once,
            data: String,
        }

        impl StringAsyncReader {
            fn new<P: Into<String>>(data: P) -> Self {
                Self {
                    once: std::sync::Once::new(),
                    data: data.into(),
                }
            }
        }

        impl AsyncRead for StringAsyncReader {
            #[inline]
            fn poll_read(
                self: Pin<&mut Self>,
                _cx: &mut Context<'_>,
                buf: &mut ReadBuf<'_>,
            ) -> Poll<io::Result<()>> {
                self.once.call_once(|| {
                    buf.put_slice(self.data.as_bytes());
                });
                Poll::Ready(Ok(()))
            }
        }

        fn get_temp_path(name: &str) -> PathBuf {
            PathBuf::new().join(std::env::temp_dir()).join(name)
        }

        fn safe_remove(p: &PathBuf) -> Result<(), std::io::Error> {
            if !p.exists() {
                return Ok(());
            }

            if p.is_dir() {
                std::fs::remove_dir_all(p)
            } else {
                std::fs::remove_file(p)
            }
        }
    }
}
