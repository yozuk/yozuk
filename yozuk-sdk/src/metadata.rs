use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Metadata {
    Docs { url: Url },
    Share { url: Url },
    Value { value: Value },
}

impl Metadata {
    pub fn docs<T>(url: T) -> Result<Self, url::ParseError>
    where
        T: AsRef<str>,
    {
        Ok(Self::Docs {
            url: Url::parse(url.as_ref())?,
        })
    }

    pub fn share<T>(url: T) -> Result<Self, url::ParseError>
    where
        T: AsRef<str>,
    {
        Ok(Self::Share {
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
}
