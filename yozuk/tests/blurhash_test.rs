#![cfg(feature = "yozuk-skill-blurhash")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn blurhash() {
    assert_eq!(
        cmd(tk!(["LlMF%n00%#MwS|WCWEM{R*bbWBbH"])),
        CommandArgs::new().add_args(["yozuk-skill-blurhash", "LlMF%n00%#MwS|WCWEM{R*bbWBbH"])
    );
}
