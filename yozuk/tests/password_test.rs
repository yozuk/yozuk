#![cfg(feature = "yozuk-skill-password")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn test_password_generation() {
    assert_eq!(
        cmd(tk!(["pwgen"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-password"]))
    );
    assert_eq!(
        cmd(tk!(["password"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-password"]))
    );
    assert_eq!(
        cmd(tk!(["generate", "password"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-password"]))
    );
    assert_eq!(
        cmd(tk!(["New", "password"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-password"]))
    );
}
