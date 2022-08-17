use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Metadata {
    Link { title: String, url: Url },
    Docs { url: Url },
    Value { value: Value },
    Color { color: String },
}

impl Metadata {
    pub fn link<T, U>(title: T, url: U) -> Result<Self, url::ParseError>
    where
        T: Into<String>,
        U: AsRef<str>,
    {
        Ok(Self::Link {
            title: title.into(),
            url: Url::parse(url.as_ref())?,
        })
    }

    pub fn docs<T>(url: T) -> Result<Self, url::ParseError>
    where
        T: AsRef<str>,
    {
        Ok(Self::Docs {
            url: Url::parse(url.as_ref())?,
        })
    }

    pub fn value<T>(value: T) -> Self
    where
        T: Into<Value>,
    {
        Self::Value {
            value: value.into(),
        }
    }

    pub fn color<T>(color: T) -> Self
    where
        T: Into<String>,
    {
        Self::Color {
            color: color.into(),
        }
    }
}
