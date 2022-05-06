#![cfg(feature = "yozuk-skill-digest")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn digest_from_args() {
    assert_eq!(
        cmd(tk!(["Hello World!", "to", "md5"])),
        CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--input",
            "Hello World!",
            "--algorithm",
            "md5"
        ])
    );
    assert_eq!(
        cmd(tk!(["😍😗😋", "into", "sha2"])),
        CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--input",
            "😍😗😋",
            "--algorithm",
            "sha2"
        ])
    );
    assert_eq!(
        cmd(tk!([
            "quick brown fox jumps over the lazy dog",
            "to",
            "sha3-256"
        ])),
        CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--input",
            "quick brown fox jumps over the lazy dog",
            "--algorithm",
            "sha3-256"
        ])
    );
}

#[test]
fn digest_from_stream() {
    assert_eq!(
        cmd(tk!(["md5"])),
        CommandArgs::new().add_args(["yozuk-skill-digest", "--algorithm", "md5"])
    );
    assert_eq!(
        cmd(tk!(["sha2", "md5"])),
        CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--algorithm",
            "sha2",
            "--algorithm",
            "md5"
        ])
    );
}
