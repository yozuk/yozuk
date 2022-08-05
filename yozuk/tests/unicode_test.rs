#![cfg(feature = "yozuk-skill-unicode")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn ascii() {
    assert_eq!(
        cmd(tk!(["a", "b", "c"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-unicode", "a", "b", "c"]))
    );
}

#[test]
fn emoji() {
    assert_eq!(
        cmd(tk!(["🏳️‍🌈", "🏳️‍⚧️", "👩🏽‍👩🏽‍👦🏽‍👦🏽"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-unicode", "🏳️‍🌈", "🏳️‍⚧️", "👩🏽‍👩🏽‍👦🏽‍👦🏽"]))
    );
}
