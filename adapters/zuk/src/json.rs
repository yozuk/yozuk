use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use yozuk_sdk::prelude::*;

#[derive(Clone, Deserialize)]
pub struct JsonInput {
    pub tokens: Vec<Token>,
}

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum JsonResult<'a> {
    Ok { output: &'a Output },
    Fail { outputs: &'a [Output] },
    Commands { commands: &'a [CommandArgs] },
    NoCommand { suggest: Option<Cow<'a, str>> },
    Error { message: &'a str },
}
