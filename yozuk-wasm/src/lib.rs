#![cfg(target_arch = "wasm32")]

use mediatype::media_type;
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;
use std::mem;
use std::ops::DerefMut;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

lazy_static::lazy_static! {
    static ref STREAMS: Mutex<Vec<Box<[u8]>>> = Mutex::new(Vec::new());
}

#[wasm_bindgen]
pub fn push_stream(buffer: Box<[u8]>) {
    STREAMS.lock().unwrap().push(buffer);
}

#[wasm_bindgen]
pub fn exec(command: &str, i18n: &str) -> Result<String, JsValue> {
    let streams = mem::take(STREAMS.lock().unwrap().deref_mut());
    let input = JsonInput {
        tokens: Tokenizer::new().tokenize(command),
        i18n: serde_json::from_str(i18n).unwrap_or_default(),
    };
    let result = run(input, streams);
    Ok(serde_json::to_string(&result).unwrap())
}

fn run(input: JsonInput, buffer: Vec<Box<[u8]>>) -> JsonResult {
    let zuk = Yozuk::builder().build();

    let mut streams = buffer
        .into_iter()
        .map(|data| InputStream::new(Cursor::new(data), media_type!(APPLICATION / OCTET_STREAM)))
        .collect::<Vec<_>>();

    let commands = zuk.get_commands(&input.tokens, &streams);
    if commands.is_empty() {
        return JsonResult::NoCommand;
    }

    match zuk.run_commands(commands, &mut streams, Some(&input.i18n)) {
        Ok(outputs) => JsonResult::Ok { outputs },
        Err(errors) => JsonResult::Fail { outputs: errors },
    }
}

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum JsonResult {
    Ok { outputs: Vec<Output> },
    Fail { outputs: Vec<Output> },
    NoCommand,
    Error { message: String },
}

#[derive(Clone, Deserialize)]
pub struct JsonInput {
    pub tokens: Vec<Token>,
    #[serde(default)]
    pub i18n: I18n,
}
