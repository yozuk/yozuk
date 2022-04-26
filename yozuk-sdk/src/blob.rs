use blake2::{digest::consts::U12, Blake2b, Digest};
use bytes::Bytes;
use std::str;

type Blake2b96 = Blake2b<U12>;

#[derive(Debug, Clone)]
pub struct Blob {
    id: String,
    data: Bytes,
}

impl Blob {
    pub fn new() -> Self {
        Self::from(Bytes::new())
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn data(&self) -> &Bytes {
        &self.data
    }

    pub fn as_utf8(&self) -> &str {
        if let Ok(str) = str::from_utf8(&self.data) {
            str
        } else {
            ""
        }
    }
}

impl Default for Blob {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Blob {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<Bytes> for Blob {
    fn from(data: Bytes) -> Self {
        let mut hasher = Blake2b96::new();
        hasher.update(&data);
        let id = base64::encode_config(&hasher.finalize(), base64::URL_SAFE);
        Self { id, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blob() {
        let blob = Blob::new();
        assert_eq!(blob.id(), "uOHdo6wKo4IK0pkL");

        let blob = Blob::from(Bytes::from_static(b"Hello World!"));
        assert_eq!(blob.id(), "VVg4lZh3I00ZfiRa");
        assert_eq!(blob.data().as_ref(), b"Hello World!");
        assert_eq!(blob.as_utf8(), "Hello World!");
    }
}
