#![cfg(all(feature = "modelgen", feature = "yozuk-skill-base64"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn encode() {
    assert_eq!(
        cmd(tk!(["Hello World!", "to", "Base64"])),
        CommandArgs::new()
            .add_args(["yozuk-skill-base64", "--mode", "encode"])
            .add_data([String::from("Hello World!")])
    );
    assert_eq!(
        cmd(tk!(["😍😗😋", "to", "Base64"])),
        CommandArgs::new()
            .add_args(["yozuk-skill-base64", "--mode", "encode"])
            .add_data([String::from("😍😗😋")])
    );
    assert_eq!(
        cmd(tk!([
            "quick brown fox jumps over the lazy dog",
            "to",
            "Base64"
        ])),
        CommandArgs::new()
            .add_args(["yozuk-skill-base64", "--mode", "encode"])
            .add_data([String::from("quick brown fox jumps over the lazy dog")])
    );
}

#[test]
fn decode() {
    assert_eq!(
        cmd(tk!(["KAAoAAAdmgCEzO0ZOVlteYWIZKzx"])),
        CommandArgs::new()
            .add_args(["yozuk-skill-base64", "--mode", "decode"])
            .add_data([String::from("KAAoAAAdmgCEzO0ZOVlteYWIZKzx")])
    );
}
