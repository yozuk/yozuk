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
    pub fn value<T>(value: T) -> Self
    where
        T: Into<Value>,
    {
        Self::Value {
            value: value.into(),
        }
    }
}
