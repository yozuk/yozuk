use crate::display::*;
use crate::highlight::*;
use crate::serde_bytes::{deserialize_bytes, serialize_bytes};
use bytes::Bytes;
use mediatype::{media_type, MediaTypeBuf};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Block {
    Comment(Comment),
    Data(Data),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comment {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,

    pub text: String,
    pub media_type: MediaTypeBuf,
}

impl From<Comment> for Block {
    fn from(block: Comment) -> Self {
        Self::Comment(block)
    }
}

impl Comment {
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
            title: String::new(),
            text: String::new(),
            media_type: media_type!(TEXT / PLAIN).into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Data {
    #[serde(
        serialize_with = "serialize_bytes",
        deserialize_with = "deserialize_bytes"
    )]
    pub data: Bytes,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub file_name: String,

    pub media_type: MediaTypeBuf,

    #[serde(skip_serializing_if = "DisplaySuggestion::is_default")]
    pub display: DisplaySuggestion,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<Highlight>,
}

impl From<Data> for Block {
    fn from(block: Data) -> Self {
        Self::Data(block)
    }
}

impl Data {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_data<T>(mut self, data: T) -> Self
    where
        T: Into<Bytes>,
    {
        self.data = data.into();
        self
    }

    pub fn set_text_data<T>(mut self, text: T) -> Self
    where
        T: Into<String>,
    {
        self.data = text.into().into();
        self.media_type = media_type!(TEXT / PLAIN).into();
        self
    }

    pub fn set_highlighted_text_data<T>(mut self, text: T, highlighter: &Highlighter) -> Self
    where
        T: Into<String>,
    {
        let (plain, highlights) = highlighter.highlight(&text.into());
        self.data = plain.into();
        self.media_type = media_type!(TEXT / PLAIN).into();
        self.highlights = highlights;
        self
    }

    pub fn set_json_data<T>(mut self, json: &T) -> Result<Self, serde_json::Error>
    where
        T: serde::Serialize,
    {
        self.data = serde_json::to_string_pretty(json)?.into();
        self.media_type = media_type!(APPLICATION / JSON).into();
        Ok(self)
    }

    pub fn set_title<T>(mut self, title: T) -> Self
    where
        T: Into<String>,
    {
        self.title = title.into();
        self
    }

    pub fn set_file_name<T>(mut self, file_name: T) -> Self
    where
        T: Into<String>,
    {
        self.file_name = file_name.into();
        self
    }

    pub fn set_media_type<T>(mut self, media_type: T) -> Self
    where
        T: Into<MediaTypeBuf>,
    {
        self.media_type = media_type.into();
        self
    }

    pub fn set_display<T>(mut self, display: T) -> Self
    where
        T: Into<DisplaySuggestion>,
    {
        self.display = display.into();
        self
    }

    pub fn set_highlights<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = Highlight>,
    {
        self.highlights = iter.into_iter().collect();
        self
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            data: Bytes::new(),
            title: String::new(),
            file_name: String::new(),
            media_type: media_type!(APPLICATION / OCTET_STREAM).into(),
            display: Default::default(),
            highlights: Vec::new(),
        }
    }
}
