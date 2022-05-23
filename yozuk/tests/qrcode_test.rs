#![cfg(feature = "yozuk-skill-qrcode")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn encode() {
    assert_eq!(
        cmd(tk!(["Hello World!", "to", "QRCode"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-qrcode", "Hello World!"]))
    );
    assert_eq!(
        cmd(tk!(["😍😗😋", "to", "QR"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-qrcode", "😍😗😋"]))
    );
    assert_eq!(
        cmd(tk!([
            "quick brown fox jumps over the lazy dog",
            "to",
            "qrcode"
        ])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-qrcode",
            "quick brown fox jumps over the lazy dog"
        ]))
    );
}
