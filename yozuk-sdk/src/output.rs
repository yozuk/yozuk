use crate::block::Block;
use crate::serde_bytes::{deserialize_bytes, serialize_bytes};
use bytes::Bytes;
use mediatype::{media_type, MediaTypeBuf};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::str;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Output {
    pub module: String,
    pub sections: Vec<Section>,
    pub blocks: Vec<Block>,
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
    pub attrs: HashMap<String, Value>,
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
            attrs: HashMap::new(),
        }
    }

    pub fn kind(mut self, kind: SectionKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn attr<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<Value>,
    {
        self.attrs.insert(key.into(), value.into());
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

#[non_exhaustive]
#[derive(Debug)]
pub enum CommandError {
    Output(Output),
    Error(anyhow::Error),
}

impl CommandError {
    pub fn into_output<T>(self, module: T) -> Output
    where
        T: Into<String>,
    {
        match self {
            Self::Output(output) => output,
            Self::Error(err) => Output {
                module: module.into(),
                sections: vec![Section::new(format!("{}", err), media_type!(TEXT / PLAIN))
                    .kind(SectionKind::Comment)],
                ..Default::default()
            },
        }
    }
}

impl From<Output> for CommandError {
    fn from(output: Output) -> Self {
        Self::Output(output)
    }
}

impl<T> From<T> for CommandError
where
    T: Into<anyhow::Error>,
{
    fn from(err: T) -> Self {
        Self::Error(err.into())
    }
}
