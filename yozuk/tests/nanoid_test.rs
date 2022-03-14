#![cfg(all(feature = "modelgen", feature = "yozuk-skill-nanoid"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn single_nanoid() {
    assert_eq!(
        cmd(tk!(["nanoid"])),
        CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "1"])
    );
    assert_eq!(
        cmd(tk!(["1", "NanoID"])),
        CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "1"])
    );
}

#[test]
fn multi_nanoid() {
    assert_eq!(
        cmd(tk!(["4", "nanoid"])),
        CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "4"])
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "NanoID"])),
        CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "10"])
    );
    assert_eq!(
        cmd(tk!(["generate", "999", "NanoIDs"])),
        CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "999"])
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "nanoid."])),
        CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "10"])
    );
    assert_eq!(
        cmd(tk!(["Hi", "yozuk,", "generate", "10", "nanoids."])),
        CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "10"])
    );
}
