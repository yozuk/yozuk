use crate::serde_bytes::{deserialize_bytes, serialize_bytes};
use bytes::Bytes;
use mediatype::{media_type, MediaTypeBuf};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Comment(Comment),
    Data(Data),
    Preview(Preview),
    CommandList(CommandList),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comment {
    pub title: String,
    pub text: String,
    pub media_type: MediaTypeBuf,
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

    pub file_name: String,
    pub media_type: MediaTypeBuf,
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
}

impl Default for Data {
    fn default() -> Self {
        Self {
            data: Bytes::new(),
            file_name: String::new(),
            media_type: media_type!(APPLICATION / OCTET_STREAM).into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Preview {
    #[serde(rename = "com.yozuk.preview.color")]
    Color(ColorPreview),
<<<<<<< HEAD
=======

    #[serde(rename = "com.yozuk.preview.location")]
    Location(LocationPreview),
>>>>>>> 97d9a114fa36f8ca286d336c45138c6de5a651ce
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorPreview {
<<<<<<< HEAD
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
=======
    pub color: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationPreview {
    pub latitude: f64,
    pub longitude: f64,
>>>>>>> 97d9a114fa36f8ca286d336c45138c6de5a651ce
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandList {
    pub commands: Vec<Command>,
}

impl CommandList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_command<T>(mut self, command: T) -> Self
    where
        T: Into<Command>,
    {
        self.commands.push(command.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Command {
    pub title: String,
    pub description: String,
    pub tokens: Vec<String>,
}

impl Command {
    pub fn new<I, T>(iter: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            title: String::new(),
            description: String::new(),
            tokens: iter.into_iter().map(Into::into).collect(),
        }
    }

    pub fn set_title<T>(mut self, title: T) -> Self
    where
        T: Into<String>,
    {
        self.title = title.into();
        self
    }

    pub fn set_description<T>(mut self, description: T) -> Self
    where
        T: Into<String>,
    {
        self.description = description.into();
        self
    }
}
