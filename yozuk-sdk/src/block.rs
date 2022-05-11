use super::blob::*;
use bytes::Bytes;
use mediatype::{media_type, MediaTypeBuf};
use secstr::SecUtf8;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Block {
    Comment(Comment),
    Data(Data),
    Preview(Preview),
    Spoiler(Spoiler),
    CommandList(CommandList),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comment {
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
    pub data: Blob,
    pub file_name: String,
    pub media_type: MediaTypeBuf,
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
        self.data = data.into().into();
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

    pub fn set_json_data<T>(mut self, json: &T) -> Result<Self, serde_json::Error>
    where
        T: serde::Serialize,
    {
        self.data = serde_json::to_string_pretty(json)?.into();
        self.media_type = media_type!(APPLICATION / JSON).into();
        Ok(self)
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
            data: Blob::new(),
            file_name: String::new(),
            media_type: media_type!(APPLICATION / OCTET_STREAM).into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Preview {
    #[serde(rename = "com.yozuk.preview.color")]
    Color(ColorPreview),
}

impl From<Preview> for Block {
    fn from(block: Preview) -> Self {
        Self::Preview(block)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorPreview {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandList {
    pub commands: Vec<Command>,
}

impl From<CommandList> for Block {
    fn from(block: CommandList) -> Self {
        Self::CommandList(block)
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spoiler {
    pub title: String,
    pub data: SecUtf8,
}

impl From<Spoiler> for Block {
    fn from(block: Spoiler) -> Self {
        Self::Spoiler(block)
    }
}

impl Spoiler {
    pub fn new<T, U>(title: T, data: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        Self {
            title: title.into(),
            data: data.into().into(),
        }
    }
}
