#![cfg(feature = "modelgen")]

use mediatype::media_type;
use std::io;
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

lazy_static::lazy_static! {
    pub static ref YOZUK: Yozuk = {
        let model = yozuk::modelgen(&Environment::new()).unwrap();
        Yozuk::builder().build(model)
    };
}

pub fn cmd(tokens: Vec<Token>) -> CommandArgs {
    let stream = InputStream::new(io::empty(), media_type!(APPLICATION / OCTET_STREAM));
    YOZUK.get_commands(&tokens, &[stream]).remove(0)
}
