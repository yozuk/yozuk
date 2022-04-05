#![cfg(all(feature = "modelgen", feature = "yozuk-skill-unit"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn units() {
    assert_eq!(
        cmd(tk!(["10", "km"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", "10", "--unit", "km"])
    );
    assert_eq!(
        cmd(tk!(["500.9", "GHz"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", "500.9", "--unit", "GHz"])
    );
    assert_eq!(
        cmd(tk!(["0.00001", "kg"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", "0.00001", "--unit", "kg"])
    );
    assert_eq!(
        cmd(tk!(["500", "Hz"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", "500", "--unit", "Hz"])
    );
}
