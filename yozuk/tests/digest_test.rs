#![cfg(feature = "yozuk-skill-digest")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn digest_from_args() {
    assert_eq!(
        cmd(tk!(["abcdefghijklmnopqrstuvwxyz", "to", "md4"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--input",
            "abcdefghijklmnopqrstuvwxyz",
            "--algorithm",
            "md4"
        ]))
    );
    assert_eq!(
        cmd(tk!(["Hello World!", "to", "md5"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--input",
            "Hello World!",
            "--algorithm",
            "md5"
        ]))
    );
    assert_eq!(
        cmd(tk!(["😍😗😋", "into", "sha2"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--input",
            "😍😗😋",
            "--algorithm",
            "sha2"
        ]))
    );
    assert_eq!(
        cmd(tk!([
            "quick brown fox jumps over the lazy dog",
            "to",
            "sha3-256"
        ])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--input",
            "quick brown fox jumps over the lazy dog",
            "--algorithm",
            "sha3-256"
        ]))
    );
    assert_eq!(
        cmd(tk!([
            "Hello World!",
            "to",
            "md5",
            "with",
            "multihash",
            "prefix"
        ])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--input",
            "Hello World!",
            "--algorithm",
            "md5",
            "--multihash"
        ]))
    );
    assert_eq!(
        cmd(tk!([
            "quick brown fox jumps over the lazy dog",
            "to",
            "sha3-256",
            "multi"
        ])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--input",
            "quick brown fox jumps over the lazy dog",
            "--algorithm",
            "sha3-256",
            "--multihash"
        ]))
    );
}

#[test]
fn digest_from_stream() {
    assert_eq!(
        cmd(tk!(["md5"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-digest", "--algorithm", "md5"]))
    );
    assert_eq!(
        cmd(tk!(["sha2", "md5"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--algorithm",
            "sha2",
            "--algorithm",
            "md5"
        ]))
    );
}
