use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use derive_new::new;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use serde::de::{Deserialize as DeserializeTrait, Deserializer};
use serde::ser::{Serialize as SerializeTrait, Serializer};

/// Represents a cursor for some list of data.
/// It will points to row from which we should start.
/// List should start from the beginning, if `last_id` is empty.
#[derive(Debug, Default, new, Clone, PartialEq)]
pub struct Cursor {
    /// Identifier of record from which we should fetch next bucket.
    pub(crate) last_id: Option<Uuid>,
}

/// Inner cursor structure.
/// Necessary because we have custom (de)serialization logic for cursor.
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
        S: Serializer,
    {
        // Encode data as an JSON and then encode by base64.
        let inner: CursorInner = self.into();
        let encoded =
            serde_json::to_string(&inner).map_err(|e| serde::ser::Error::custom(e.to_string()))?;
        let encoded = URL_SAFE.encode(encoded);

        serializer.serialize_str(&encoded)
    }
}

impl<'de> DeserializeTrait<'de> for Cursor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vis;
        impl serde::de::Visitor<'_> for Vis {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a base64 string")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                let v = URL_SAFE
                    .decode(v)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                Ok(v)
            }
        }
        let encoded = deserializer.deserialize_str(Vis)?;

        let inner: CursorInner = serde_json::from_slice(&encoded)
            .map_err(|e| serde::de::Error::custom(e.to_string()))?;

        Ok(inner.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod cursor {
        use super::*;

        #[test]
        fn serialize_deserialize_with_value() {
            let expected = Cursor {
                last_id: Some(Uuid::new_v4()),
            };

            let s = serde_json::to_string(&expected).unwrap();
            let actual: Cursor = serde_json::from_str(&s).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serialize_deserialize_without_value() {
            let expected = Cursor { last_id: None };

            let s = serde_json::to_string(&expected).unwrap();
            let actual: Cursor = serde_json::from_str(&s).unwrap();

            assert_eq!(expected, actual);
        }
    }
}
