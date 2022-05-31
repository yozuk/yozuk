use once_cell::sync::OnceCell;
use std::io;
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

pub fn yozuk_global() -> &'static Yozuk {
    static INSTANCE: OnceCell<Yozuk> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Yozuk::builder()
            .add_redirection(
                tk!(["test", "command", "redirect"]),
                vec!["test", "redirect"],
            )
            .build()
    })
}

pub fn cmd(tokens: Vec<Token>) -> Option<CommandArgs> {
    let stream = InputStream::new(io::empty(), media_type!(APPLICATION / OCTET_STREAM));
    yozuk_global()
        .get_commands(&tokens, &[stream])
        .into_iter()
        .next()
}
