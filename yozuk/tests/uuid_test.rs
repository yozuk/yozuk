#![cfg(feature = "yozuk-skill-uuid")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn single_uuid() {
    assert_eq!(
        cmd(tk!(["uuid"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "1"]))
    );
    assert_eq!(
        cmd(tk!(["1", "uuid"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "1"]))
    );
}

#[test]
fn multi_uuid() {
    assert_eq!(
        cmd(tk!(["4", "uuids"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "4"]))
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "uuid"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "10"]))
    );
    assert_eq!(
        cmd(tk!(["generate", "999", "uuid"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "999"]))
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "uuids."])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "10"]))
    );
    assert_eq!(
        cmd(tk!(["Hi", "yozuk,", "generate", "10", "uuids."])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "10"]))
    );
}

#[test]
fn uppercase_uuid() {
    assert_eq!(
        cmd(tk!(["uuid", "UPPERCASE"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "1", "--upper"]))
    );
    assert_eq!(
        cmd(tk!(["1", "uuid", "upper"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "1", "--upper"]))
    );
}

#[test]
fn lowercase_uuid() {
    assert_eq!(
        cmd(tk!(["uuid", "lowercase"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "1"]))
    );
    assert_eq!(
        cmd(tk!(["1", "uuid", "lower"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "1"]))
    );
}
