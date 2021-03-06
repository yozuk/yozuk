#![cfg(feature = "yozuk-skill-punycode")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn decode_punycode() {
    assert_eq!(
        cmd(tk!(["xn--cookie!-1d84f"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-punycode",
            "--mode",
            "decode",
            "xn--cookie!-1d84f"
        ]))
    );
    assert_eq!(
        cmd(tk!(["xn--li8h.and.xn--ri8h", "xn--mushroom-bd25gia"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-punycode",
            "--mode",
            "decode",
            "xn--li8h.and.xn--ri8h",
            "xn--mushroom-bd25gia"
        ]))
    );
}

#[test]
fn encode_punycode() {
    assert_eq!(
        cmd(tk!(["๐ฆ.org"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-punycode", "--mode", "encode", "๐ฆ.org",]))
    );
    assert_eq!(
        cmd(tk!(["๐ช.com", "cookie.ใในใ"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-punycode",
            "--mode",
            "encode",
            "๐ช.com",
            "cookie.ใในใ"
        ]))
    );
}
