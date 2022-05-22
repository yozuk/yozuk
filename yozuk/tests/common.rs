use std::io;
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

lazy_static::lazy_static! {
    pub static ref YOZUK: Yozuk = {
        Yozuk::builder()
            .add_redirection(tk!(["test", "command", "redirect"]), vec!["test", "redirect"])
            .build()
    };
}

pub fn cmd(tokens: Vec<Token>) -> Option<CommandArgs> {
    let stream = InputStream::new(io::empty(), media_type!(APPLICATION / OCTET_STREAM));
    YOZUK.get_commands(&tokens, &[stream]).into_iter().next()
}
