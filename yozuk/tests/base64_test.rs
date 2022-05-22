#![cfg(feature = "yozuk-skill-base64")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn encode() {
    assert_eq!(
        cmd(tk!(["Hello World!", "to", "Base64"])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-base64", "--mode", "encode"])
                .add_data([String::from("Hello World!")])
        )
    );
    assert_eq!(
        cmd(tk!(["😍😗😋", "to", "Base64"])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-base64", "--mode", "encode"])
                .add_data([String::from("😍😗😋")])
        )
    );
    assert_eq!(
        cmd(tk!([
            "quick brown fox jumps over the lazy dog",
            "to",
            "Base64"
        ])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-base64", "--mode", "encode"])
                .add_data([String::from("quick brown fox jumps over the lazy dog")])
        )
    );
}

#[test]
fn decode() {
    assert_eq!(
        cmd(tk!(["KAAoAAAdmgCEzO0ZOVlteYWIZKzx"])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-base64", "--mode", "decode"])
                .add_data([String::from("KAAoAAAdmgCEzO0ZOVlteYWIZKzx")])
        )
    );
    assert_eq!(
        cmd(tk!(["8J+Mjw=="])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-base64", "--mode", "decode"])
                .add_data([String::from("8J+Mjw==")])
        )
    );
    assert_eq!(
        cmd(tk!(["eWFtbA=="])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-base64", "--mode", "decode"])
                .add_data([String::from("eWFtbA==")])
        )
    );
    assert_eq!(
        cmd(tk!(["YWE="])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-base64", "--mode", "decode"])
                .add_data([String::from("YWE=")])
        )
    );
}
