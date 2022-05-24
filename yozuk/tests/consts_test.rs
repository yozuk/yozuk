#![cfg(feature = "yozuk-skill-consts")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn pi() {
    assert_eq!(
        cmd(tk!(["pi"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "pi"]))
    );
    assert_eq!(
        cmd(tk!(["PI"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "pi"]))
    );
    assert_eq!(
        cmd(tk!(["π"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "pi"]))
    );
}

#[test]
fn speed_of_light() {
    assert_eq!(
        cmd(tk!(["Speed", "of", "Light"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "speed-of-light"]))
    );
    assert_eq!(
        cmd(tk!(["speed", "of", "light", "in", "vacuum"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "speed-of-light"]))
    );
}
