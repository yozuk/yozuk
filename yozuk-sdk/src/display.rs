use serde_derive::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DisplaySuggestion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<BinaryDisplay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageDisplay>,
}

impl DisplaySuggestion {
    pub fn is_default(&self) -> bool {
        *self == Default::default()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BinaryDisplay {
    Viewer,
    Base64,
    Hex,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ImageDisplay {
    Smooth,
    Pixelated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Highlight {
    pub kind: HighlightKind,
    pub range: Range<usize>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum HighlightKind {
    Value,
}
