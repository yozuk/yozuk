#![cfg(all(feature = "modelgen", feature = "yozuk-skill-dice"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn simple() {
    assert_eq!(
        cmd(tk!(["2d6"])),
        CommandArgs::new().add_args(["yozuk-skill-dice", "2d6"])
    );
    assert_eq!(
        cmd(tk!(["100d1000"])),
        CommandArgs::new().add_args(["yozuk-skill-dice", "100d1000"])
    );
    assert_eq!(
        cmd(tk!(["6d"])),
        CommandArgs::new().add_args(["yozuk-skill-dice", "6d"])
    );
}

#[test]
fn operators() {
    assert_eq!(
        cmd(tk!(["(", "2d6", "+", "5d100", ")", "*", "4d10", "+100"])),
        CommandArgs::new().add_args(["yozuk-skill-dice", "(2d6+5d100)*4d10+100"])
    );
}
