#![cfg(feature = "yozuk-skill-hex")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn encode() {
    assert_eq!(
        cmd(tk!(["Hello World!", "to", "hex"])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-hex", "--mode", "encode"])
                .add_data([String::from("Hello World!")])
        )
    );
    assert_eq!(
        cmd(tk!(["😍😗😋", "to", "hex"])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-hex", "--mode", "encode"])
                .add_data([String::from("😍😗😋")])
        )
    );
    assert_eq!(
        cmd(tk!([
            "quick brown fox jumps over the lazy dog",
            "to",
            "hex"
        ])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-hex", "--mode", "encode"])
                .add_data([String::from("quick brown fox jumps over the lazy dog")])
        )
    );
}

#[test]
fn decode() {
    assert_eq!(
        cmd(tk!(["717569636b2062726f776e20666978206a756d6f73"])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-hex", "--mode", "decode"])
                .add_data([String::from("717569636b2062726f776e20666978206a756d6f73")])
        )
    );
    assert_eq!(
        cmd(tk!(["717569636bf09fa68a"])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-hex", "--mode", "decode"])
                .add_data([String::from("717569636bf09fa68a")])
        )
    );
}
