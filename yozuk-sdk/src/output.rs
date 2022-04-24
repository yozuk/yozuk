use crate::block::{self, Block};
use serde_derive::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Output {
    pub title: String,
    pub blocks: Vec<Block>,
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
