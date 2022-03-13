use crate::serde_bytes::{deserialize_bytes_vec, serialize_bytes_vec};
use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommandArgs {
    pub args: Vec<String>,

    #[serde(
        serialize_with = "serialize_bytes_vec",
        deserialize_with = "deserialize_bytes_vec"
    )]
    pub data: Vec<Bytes>,
}

impl CommandArgs {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_args<T>(mut self, item: T) -> Self
    where
        T: IntoArgs<String>,
    {
        self.args.append(&mut item.into_args());
        self
    }

    pub fn add_args_iter<T, I>(mut self, iter: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        self.args
            .append(&mut iter.into_iter().map(Into::into).collect());
        self
    }

    pub fn add_data<T>(mut self, item: T) -> Self
    where
        T: IntoArgs<Bytes>,
    {
        self.data.append(&mut item.into_args());
        self
    }

    pub fn add_data_iter<T, I>(mut self, iter: I) -> Self
    where
        T: Into<Bytes>,
        I: IntoIterator<Item = T>,
    {
        self.data
            .append(&mut iter.into_iter().map(Into::into).collect());
        self
    }
}

impl<T> FromIterator<T> for CommandArgs
where
    T: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            args: iter.into_iter().map(Into::into).collect(),
            data: vec![],
        }
    }
}

pub trait IntoArgs<T> {
    fn into_args(self) -> Vec<T>;
}

impl<const N: usize, T> IntoArgs<String> for [T; N]
where
    T: Into<String>,
{
    fn into_args(self) -> Vec<String> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<const N: usize, T> IntoArgs<String> for Option<[T; N]>
where
    T: Into<String>,
{
    fn into_args(self) -> Vec<String> {
        if let Some(args) = self {
            args.into_iter().map(Into::into).collect()
        } else {
            vec![]
        }
    }
}

impl<const N: usize, T> IntoArgs<Bytes> for [T; N]
where
    T: Into<Bytes>,
{
    fn into_args(self) -> Vec<Bytes> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<const N: usize, T> IntoArgs<Bytes> for Option<[T; N]>
where
    T: Into<Bytes>,
{
    fn into_args(self) -> Vec<Bytes> {
        if let Some(args) = self {
            args.into_iter().map(Into::into).collect()
        } else {
            vec![]
        }
    }
}
