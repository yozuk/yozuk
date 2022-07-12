#![cfg(feature = "yozuk-skill-numeric")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn base_convertion() {
    assert_eq!(
        cmd(tk!(["0xff"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-numeric", "0xff"]))
    );
    assert_eq!(
        cmd(tk!(["0b11111111"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-numeric", "0b11111111"]))
    );
    assert_eq!(
        cmd(tk!(["0o377"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-numeric", "0o377"]))
    );
    assert_eq!(
        cmd(tk!(["255"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-numeric", "255"]))
    );
}
