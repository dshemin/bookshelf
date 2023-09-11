use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use derive_new::new;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use serde::ser::{Serialize as SerializeTrait, Serializer};
use serde::de::{Deserialize as DeserializeTrait, Deserializer};

pub mod storage;

/// Max number of data per request to DB.
pub(crate) const LIMIT: usize = 25;

#[derive(Debug, Default, new, Clone, PartialEq)]
pub struct Cursor {
    last_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize)]
struct CursorInner {
    last_id: Option<Uuid>,
}


impl From<CursorInner> for Cursor {
    fn from(value: CursorInner) -> Self {
        Self {
            last_id: value.last_id,
        }
    }
}

impl From<&Cursor> for CursorInner {
    fn from(value: &Cursor) -> Self {
        Self {
            last_id: value.last_id,
        }
    }
}

impl SerializeTrait for Cursor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        // Encode data as an JSON and then encode by base64.
        let inner: CursorInner = self.into();
        let encoded = serde_json::to_string(&inner).map_err(|e| {
            serde::ser::Error::custom(e.to_string())
        })?;
        let encoded = URL_SAFE.encode(encoded);

        serializer.serialize_str(&encoded)
    }
}

impl<'de> DeserializeTrait<'de> for Cursor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        struct Vis;
        impl serde::de::Visitor<'_> for Vis {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a base64 string")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                let v = URL_SAFE.decode(v).map_err(|e| {
                    serde::de::Error::custom(e.to_string())
                })?;
                Ok(v)
            }
        }
        let encoded = deserializer.deserialize_str(Vis)?;

        let inner: CursorInner = serde_json::from_slice(&encoded).map_err(|e| {
            serde::de::Error::custom(e.to_string())
        })?;

        Ok(inner.into())
    }
}

#[derive(Debug, Serialize, Default)]
pub struct PaginatedData<T> {
    data: Vec<T>,
    cursor: Option<Cursor>,
}

#[cfg(test)]
mod test {
    use super::*;

    mod cursor {
        use super::*;

        #[test]
        fn serialize_deserialize() {
            let expected = Cursor {
                last_id: Some(Uuid::new_v4()),
            };

            let s = serde_json::to_string(&expected).unwrap();
            let actual: Cursor = serde_json::from_str(&s).unwrap();

            assert_eq!(expected, actual);
        }
    }
}
