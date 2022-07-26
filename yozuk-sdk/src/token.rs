use crate::serde_bytes::{deserialize_bytes, serialize_bytes};
use bytes::Bytes;
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_str: Option<String>,

    #[serde(default = "String::new")]
    pub tag: String,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            data: Bytes::new(),
            raw_str: None,
            tag: String::new(),
        }
    }
}

impl Token {
    pub fn as_str(&self) -> &str {
        if let Ok(str) = str::from_utf8(&self.data) {
            str
        } else {
            ""
        }
    }
}
