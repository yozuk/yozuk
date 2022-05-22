#![cfg(feature = "yozuk-skill-dice")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn simple() {
    assert_eq!(
        cmd(tk!(["dice"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-dice", "1d6"]))
    );
    assert_eq!(
        cmd(tk!(["roll", "die"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-dice", "1d6"]))
    );
    assert_eq!(
        cmd(tk!(["roll", "10", "dice"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-dice", "10d6"]))
    );
}

#[test]
fn simple_notation() {
    assert_eq!(
        cmd(tk!(["2d6"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-dice", "2d6"]))
    );
    assert_eq!(
        cmd(tk!(["100d1000"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-dice", "100d1000"]))
    );
    assert_eq!(
        cmd(tk!(["6d"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-dice", "6d"]))
    );
}

#[test]
fn operators() {
    assert_eq!(
        cmd(tk!(["(", "2d6", "+", "5d100", ")", "*", "4d10", "+100"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-dice", "(2d6+5d100)*4d10+100"]))
    );
}
