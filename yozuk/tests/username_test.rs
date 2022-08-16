#![cfg(feature = "yozuk-skill-username")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn single_username() {
    assert_eq!(
        cmd(tk!(["username"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "1"]))
    );
    assert_eq!(
        cmd(tk!(["1", "username"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "1"]))
    );
}

#[test]
fn multi_username() {
    assert_eq!(
        cmd(tk!(["4", "usernames"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "4"]))
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "username"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "10"]))
    );
    assert_eq!(
        cmd(tk!(["generate", "999", "username"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "999"]))
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "usernames."])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "10"]))
    );
    assert_eq!(
        cmd(tk!(["Hi", "yozuk,", "generate", "10", "usernames."])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "10"]))
    );
}

#[test]
fn uppercase_username() {
    assert_eq!(
        cmd(tk!(["username", "UPPERCASE"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "1", "--upper"]))
    );
    assert_eq!(
        cmd(tk!(["1", "username", "upper"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "1", "--upper"]))
    );
}

#[test]
fn lowercase_username() {
    assert_eq!(
        cmd(tk!(["username", "lowercase"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "1"]))
    );
    assert_eq!(
        cmd(tk!(["1", "username", "lower"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-username", "-n", "1"]))
    );
}
