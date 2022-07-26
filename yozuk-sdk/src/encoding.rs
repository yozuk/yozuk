use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RawEncoding {
    Base64,
    Base64Url,
    Hex,
}

impl RawEncoding {
    pub fn all() -> [Self; 3] {
        [Self::Base64, Self::Base64Url, Self::Hex]
    }
}
