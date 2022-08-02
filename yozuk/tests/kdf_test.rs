#![cfg(feature = "yozuk-skill-kdf")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn hash() {
    assert_eq!(
        cmd(tk!(["abcdefghijklmnopqrstuvwxyz", "to", "scrypt"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-kdf",
            "--input",
            "abcdefghijklmnopqrstuvwxyz",
            "--algorithm",
            "scrypt"
        ]))
    );
    assert_eq!(
        cmd(tk!(["😍😗😋", "into", "bcrypt"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-kdf",
            "--input",
            "😍😗😋",
            "--algorithm",
            "bcrypt"
        ]))
    );
}
