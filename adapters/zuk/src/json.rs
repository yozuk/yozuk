use serde_derive::Serialize;
use yozuk_sdk::prelude::*;

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum JsonResult<'a> {
    Ok { output: &'a Output },
    Fail { output: &'a Output },
    Commands { commands: &'a [CommandArgs] },
    Suggest { suggest: &'a str },
    Error { message: &'a str },
}
