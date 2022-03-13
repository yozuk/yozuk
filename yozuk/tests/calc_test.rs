#![cfg(all(feature = "modelgen", feature = "yozuk-skill-calc"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn single_calc() {
    assert_eq!(
        cmd(tk!(["1 + 1"])),
        CommandArgs::new().add_args(["yozuk-skill-calc", "1 + 1"])
    );
    assert_eq!(
        cmd(tk!(["(100 + 200) * 5555.234"])),
        CommandArgs::new().add_args(["yozuk-skill-calc", "(100 + 200) * 5555.234"])
    );
}

#[test]
fn multi_calc() {
    assert_eq!(
        cmd(tk!(["1 +", "1"])),
        CommandArgs::new().add_args(["yozuk-skill-calc", "1 +1"])
    );
    assert_eq!(
        cmd(tk!(["(100", " + 200) ", "*", " 5555.234"])),
        CommandArgs::new().add_args(["yozuk-skill-calc", "(100 + 200) * 5555.234"])
    );
}
