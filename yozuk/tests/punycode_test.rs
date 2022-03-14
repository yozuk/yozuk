#![cfg(all(feature = "modelgen", feature = "yozuk-skill-punycode"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn decode_punycode() {
    assert_eq!(
        cmd(tk!(["xn--cookie!-1d84f"])),
        CommandArgs::new().add_args([
            "yozuk-skill-punycode",
            "--mode",
            "decode",
            "xn--cookie!-1d84f"
        ])
    );
    assert_eq!(
        cmd(tk!(["xn--li8h.and.xn--ri8h", "xn--mushroom-bd25gia"])),
        CommandArgs::new().add_args([
            "yozuk-skill-punycode",
            "--mode",
            "decode",
            "xn--li8h.and.xn--ri8h",
            "xn--mushroom-bd25gia"
        ])
    );
}
