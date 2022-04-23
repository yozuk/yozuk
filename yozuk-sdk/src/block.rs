use mediatype::{media_type, MediaTypeBuf};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub location: Option<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum BlockElement {
    Comment(Comment),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comment {
    pub text: String,
    pub media_type: MediaTypeBuf,
}

impl Comment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_text<T>(mut self, text: T) -> Self
    where
        T: Into<String>,
    {
        self.text = text.into();
        self
    }

    pub fn set_media_type<T>(mut self, media_type: T) -> Self
    where
        T: Into<MediaTypeBuf>,
    {
        self.media_type = media_type.into();
        self
    }
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            text: String::new(),
            media_type: media_type!(TEXT / PLAIN).into(),
        }
    }
}
