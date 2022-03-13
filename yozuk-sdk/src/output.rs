use crate::serde_bytes::{deserialize_bytes, serialize_bytes};
use bytes::Bytes;
use mediatype::MediaTypeBuf;
use serde_derive::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Output {
    pub module: String,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Section {
    #[serde(
        serialize_with = "serialize_bytes",
        deserialize_with = "deserialize_bytes"
    )]
    pub data: Bytes,

    pub media_type: MediaTypeBuf,
    pub kind: SectionKind,
}

impl Section {
    pub fn new<B, M>(data: B, media_type: M) -> Self
    where
        B: Into<Bytes>,
        M: Into<MediaTypeBuf>,
    {
        Self {
            data: data.into(),
            media_type: media_type.into(),
            kind: SectionKind::Value,
        }
    }

    pub fn kind(mut self, kind: SectionKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn as_utf8(&self) -> &str {
        if let Ok(str) = str::from_utf8(&self.data) {
            str
        } else {
            ""
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SectionKind {
    Value,
    Comment,
}

impl Default for SectionKind {
    fn default() -> Self {
        Self::Value
    }
}
