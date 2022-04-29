use crate::serde_bytes::{deserialize_bytes, serialize_bytes};
use bytes::Bytes;
use mediatype::{media_type, MediaTypeBuf};
use serde_derive::{Deserialize, Serialize};
use std::str;

#[macro_export]
macro_rules! tk {
    ([$($data:expr $(; $tag:literal)?),+]) => {
        vec![$(tk!($data $(; $tag)?)),+]
    };
    ([$($data:expr $(; $tag:literal)?),+], $madia_type:expr) => {
        vec![$(tk!($data $(; $tag)?, $madia_type)),+]
    };
    ($data:expr) => {
        $crate::token::Token{
            data: $data.into(),
            media_type: "text/plain".parse().unwrap(),
            ..Default::default()
        }
    };
    ($data:expr; $tag:literal) => {
        $crate::token::Token{
            data: $data.into(),
            tag: $tag.into(),
            ..Default::default()
        }
    };
    ($data:expr, $madia_type:expr) => {
        $crate::token::Token{
            data: $data.into(),
            media_type: $madia_type
                .parse()
                .unwrap(),
            ..Default::default()
        }
    };
    ($data:expr; $tag:literal, $madia_type:expr) => {
        $crate::token::Token{
            data: $data.into(),
            media_type: $madia_type
                .parse()
                .unwrap(),
            tag: $tag.into()
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    #[serde(
        serialize_with = "serialize_bytes",
        deserialize_with = "deserialize_bytes"
    )]
    pub data: Bytes,

    #[serde(default = "media_type_default")]
    pub media_type: MediaTypeBuf,

    #[serde(default = "String::new")]
    pub tag: String,
}

fn media_type_default() -> MediaTypeBuf {
    media_type!(TEXT / PLAIN).into()
}

impl Default for Token {
    fn default() -> Self {
        Self {
            data: Bytes::new(),
            media_type: media_type_default(),
            tag: String::new(),
        }
    }
}

impl Token {
    pub fn as_utf8(&self) -> &str {
        if let Ok(str) = str::from_utf8(&self.data) {
            str
        } else {
            ""
        }
    }
}
