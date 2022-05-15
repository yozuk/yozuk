use crate::block::{self, Block};
use crate::metadata::Metadata;
use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Output {
    pub title: String,
    pub blocks: Vec<Block>,
    pub metadata: Vec<Metadata>,
    pub mode: OutputMode,
}

impl Output {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_title<T>(mut self, title: T) -> Self
    where
        T: Into<String>,
    {
        self.title = title.into();
        self
    }

    pub fn set_mode(mut self, mode: OutputMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn add_block<T>(mut self, block: T) -> Self
    where
        T: Into<Block>,
    {
        self.blocks.push(block.into());
        self
    }

    pub fn add_blocks<T, I>(mut self, iter: I) -> Self
    where
        T: Into<Block>,
        I: IntoIterator<Item = T>,
    {
        self.blocks
            .append(&mut iter.into_iter().map(Into::into).collect());
        self
    }

    pub fn add_metadata<T>(mut self, data: T) -> Self
    where
        T: Into<Metadata>,
    {
        self.metadata.push(data.into());
        self
    }

    pub fn externalize_large_blobs(&mut self, len: usize) -> Vec<(String, Bytes)> {
        self.blocks
            .iter_mut()
            .filter_map(|block| {
                if let Block::Data(ref mut data) = block {
                    if data.data.data().map(|data| data.len()).unwrap_or(0) > len {
                        data.data.externalize()
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<T> FromIterator<T> for Output
where
    T: Into<Block>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            blocks: iter.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum OutputMode {
    Primary,
    Attachment,
}

impl Default for OutputMode {
    fn default() -> Self {
        Self::Primary
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum CommandError {
    Output(Output),
    Error(anyhow::Error),
}

impl PartialEq for CommandError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Output(lhs), Self::Output(rhs)) => lhs == rhs,
            (Self::Error(lhs), Self::Error(rhs)) => lhs.to_string() == rhs.to_string(),
            _ => false,
        }
    }
}

impl CommandError {
    pub fn into_output<T>(self, title: T) -> Output
    where
        T: Into<String>,
    {
        match self {
            Self::Output(output) => output,
            Self::Error(err) => Output {
                title: title.into(),
                blocks: vec![Block::Comment(
                    block::Comment::new().set_text(format!("{}", err)),
                )],
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
