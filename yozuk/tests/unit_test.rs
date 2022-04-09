#![cfg(all(feature = "modelgen", feature = "yozuk-skill-unit"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn units() {
    assert_eq!(
        cmd(tk!(["0.00001", "kg"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 0.00001", "--unit", "kg"])
    );
    assert_eq!(
        cmd(tk!(["0.00001kg"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 0.00001", "--unit", "kg"])
    );
    assert_eq!(
        cmd(tk!(["500", "g"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 500", "--unit", "g"])
    );
    assert_eq!(
        cmd(tk!(["500g"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 500", "--unit", "g"])
    );
    assert_eq!(
        cmd(tk!(["9999999.999", "ng"])),
        CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 9999999.999",
            "--unit",
            "ng"
        ])
    );
    assert_eq!(
        cmd(tk!(["9999999.999ng"])),
        CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 9999999.999",
            "--unit",
            "ng"
        ])
    );
    assert_eq!(
        cmd(tk!(["1000ounce"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 1000", "--unit", "ounce"])
    );
    assert_eq!(
        cmd(tk!(["1000oz"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 1000", "--unit", "oz"])
    );
    assert_eq!(
        cmd(tk!(["2.5ft"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 2.5", "--unit", "ft"])
    );
    assert_eq!(
        cmd(tk!(["10KiB"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 10", "--unit", "KiB"])
    );
    assert_eq!(
        cmd(tk!(["-100", "°F"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " -100", "--unit", "°F"])
    );
    assert_eq!(
        cmd(tk!(["100.0", "m/s"])),
        CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 100.0", "--unit", "m/s"])
    );
}
