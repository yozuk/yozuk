#![cfg(all(feature = "modelgen", feature = "yozuk-skill-uuid"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn single_uuid() {
    assert_eq!(
        cmd(tk!(["uuid"])),
        CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "1"])
    );
    assert_eq!(
        cmd(tk!(["1", "uuid"])),
        CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "1"])
    );
}

#[test]
fn multi_uuid() {
    assert_eq!(
        cmd(tk!(["4", "uuids"])),
        CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "4"])
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "uuid"])),
        CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "10"])
    );
    assert_eq!(
        cmd(tk!(["generate", "999", "uuid"])),
        CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "999"])
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "uuids."])),
        CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "10"])
    );
    assert_eq!(
        cmd(tk!(["Hi", "yozuk,", "generate", "10", "uuids."])),
        CommandArgs::new().add_args(["yozuk-skill-uuid", "-n", "10"])
    );
}
