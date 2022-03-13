#![cfg(feature = "modelgen")]

use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

lazy_static::lazy_static! {
    pub static ref YOZUK: Yozuk = {
        let model = yozuk::modelgen(&Environment::new()).unwrap();
        Yozuk::builder().build(model)
    };
}

pub fn cmd(tokens: Vec<Token>) -> CommandArgs {
    YOZUK.get_commands(&tokens).unwrap().remove(0)
}
