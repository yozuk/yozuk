#![cfg(all(feature = "modelgen", feature = "yozuk-skill-password"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn test_password_generation() {
    assert_eq!(
        cmd(tk!(["pwgen"])),
        CommandArgs::new().add_args(["yozuk-skill-password"])
    );
    assert_eq!(
        cmd(tk!(["password"])),
        CommandArgs::new().add_args(["yozuk-skill-password"])
    );
    assert_eq!(
        cmd(tk!(["generate", "password"])),
        CommandArgs::new().add_args(["yozuk-skill-password"])
    );
    assert_eq!(
        cmd(tk!(["New", "password"])),
        CommandArgs::new().add_args(["yozuk-skill-password"])
    );
}
