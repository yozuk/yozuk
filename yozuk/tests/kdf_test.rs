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
    assert_eq!(
        cmd(tk!([
            "quick brown fox jumps over the lazy dog",
            "to",
            "argon2"
        ])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-kdf",
            "--input",
            "quick brown fox jumps over the lazy dog",
            "--algorithm",
            "argon2"
        ]))
    );
    assert_eq!(
        cmd(tk!([
            "Sphinx of black quartz, judge my vow!",
            "to",
            "argon2i"
        ])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-kdf",
            "--input",
            "Sphinx of black quartz, judge my vow!",
            "--algorithm",
            "argon2i"
        ]))
    );
    assert_eq!(
        cmd(tk!(["Hello World!", "to", "argon2d"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-kdf",
            "--input",
            "Hello World!",
            "--algorithm",
            "argon2d"
        ]))
    );
}
