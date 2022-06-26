#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use mediatype::media_type;
use once_cell::sync::OnceCell;
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;
use std::mem;
use std::ops::DerefMut;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

fn global_streams() -> &'static Mutex<Vec<Box<[u8]>>> {
    static INSTANCE: OnceCell<Mutex<Vec<Box<[u8]>>>> = OnceCell::new();
    INSTANCE.get_or_init(|| Mutex::new(Vec::new()))
}

fn global_yozuk() -> &'static Yozuk {
    static INSTANCE: OnceCell<Yozuk> = OnceCell::new();
    INSTANCE.get_or_init(|| Yozuk::builder().build())
}

#[wasm_bindgen]
pub fn push_stream(buffer: Box<[u8]>) {
    global_streams().lock().unwrap().push(buffer);
}

#[wasm_bindgen]
pub fn random_suggests(amount: usize) -> String {
    serde_json::to_string(&global_yozuk().random_suggests(amount)).unwrap()
}

#[wasm_bindgen]
pub fn exec(command: &str, i18n: &str) -> Result<String, JsValue> {
    let streams = mem::take(global_streams().lock().unwrap().deref_mut());
    let input = JsonInput {
        tokens: Tokenizer::new().tokenize(command),
        i18n: serde_json::from_str(i18n).unwrap_or_default(),
    };
    let result = run(input, streams);
    Ok(serde_json::to_string(&result).unwrap())
}

fn run(input: JsonInput, buffer: Vec<Box<[u8]>>) -> JsonResult {
    let mut streams = buffer
        .into_iter()
        .map(|data| InputStream::new(Cursor::new(data), media_type!(APPLICATION / OCTET_STREAM)))
        .collect::<Vec<_>>();

    let commands = global_yozuk().get_commands(&input.tokens, &streams);
    if commands.is_empty() {
        return JsonResult::NoCommand;
    }

    match global_yozuk().run_commands(commands, &mut streams, Some(&input.i18n)) {
        Ok(outputs) => JsonResult::Ok { outputs },
        Err(outputs) => JsonResult::Fail { outputs },
    }
}

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum JsonResult {
    Ok { outputs: Vec<Output> },
    Fail { outputs: Vec<Output> },
    NoCommand,
}

#[derive(Clone, Deserialize)]
pub struct JsonInput {
    pub tokens: Vec<Token>,
    #[serde(default)]
    pub i18n: I18n,
}
