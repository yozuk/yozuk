#![cfg(feature = "yozuk-skill-nanoid")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn single_nanoid() {
    assert_eq!(
        cmd(tk!(["nanoid"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "1"]))
    );
    assert_eq!(
        cmd(tk!(["1", "NanoID"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "1"]))
    );
}

#[test]
fn multi_nanoid() {
    assert_eq!(
        cmd(tk!(["4", "nanoid"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "4"]))
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "NanoID"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "10"]))
    );
    assert_eq!(
        cmd(tk!(["generate", "999", "NanoIDs"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "999"]))
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "nanoid."])),
        Some(CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "10"]))
    );
    assert_eq!(
        cmd(tk!(["Hi", "yozuk,", "generate", "10", "nanoids."])),
        Some(CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "10"]))
    );
}
