use super::serde_bytes::{deserialize_bytes, serialize_bytes};
use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};
use std::mem;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Blob {
    Internal(
        #[serde(
            serialize_with = "serialize_bytes",
            deserialize_with = "deserialize_bytes"
        )]
        Bytes,
    ),
    External {
        #[serde(rename = "blob")]
        id: String,
    },
}

impl Blob {
    pub fn new() -> Self {
        Self::from(Bytes::new())
    }

    pub fn data(&self) -> Option<&Bytes> {
        match self {
            Self::Internal(data) => Some(data),
            _ => None,
        }
    }

    pub fn externalize(&mut self) -> Option<(String, Bytes)> {
        if matches!(self, Self::Internal(_)) {
            let id = nanoid::nanoid!();
            if let Self::Internal(data) = mem::replace(self, Self::External { id: id.clone() }) {
                return Some((id, data));
            }
        }
        None
    }
}

impl Default for Blob {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Bytes> for Blob {
    fn from(data: Bytes) -> Self {
        Self::Internal(data)
    }
}

impl From<String> for Blob {
    fn from(data: String) -> Self {
        Self::Internal(data.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blob_serde() {
        let blob: Blob = serde_json::from_str("{\"blob\": \"5XxqIJgllsQk\"}").unwrap();
        assert_eq!(
            blob,
            Blob::External {
                id: "5XxqIJgllsQk".into()
            }
        );

        let blob: Blob = serde_json::from_str("\"Hello World!\"").unwrap();
        assert_eq!(blob, Blob::Internal(Bytes::from_static(b"Hello World!")));
    }
}
