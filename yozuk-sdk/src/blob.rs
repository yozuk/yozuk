use blake2::{digest::consts::U10, Blake2b, Digest};
use bytes::Bytes;

type Blake2b80 = Blake2b<U10>;

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
}

impl Default for Blob {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Bytes> for Blob {
    fn from(data: Bytes) -> Self {
        let mut hasher = Blake2b80::new();
        hasher.update(&data);
        let id = base64::encode_config(&hasher.finalize(), base64::URL_SAFE);
        Self { id, data }
    }
}
