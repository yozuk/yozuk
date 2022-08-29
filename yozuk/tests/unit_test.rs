#![cfg(feature = "yozuk-skill-unit")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn units() {
    assert_eq!(
        cmd(tk!(["0.00001", "kg"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 0.00001",
            "--unit",
            "kg"
        ]))
    );
    assert_eq!(
        cmd(tk!(["0.00001kg"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 0.00001",
            "--unit",
            "kg"
        ]))
    );
    assert_eq!(
        cmd(tk!(["0.00001kg", "to", "oz"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 0.00001",
            "--unit",
            "kg",
            "--to",
            "oz"
        ]))
    );
    assert_eq!(
        cmd(tk!(["500", "g"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 500", "--unit", "g"]))
    );
    assert_eq!(
        cmd(tk!(["500", "g", "to", "mg"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 500",
            "--unit",
            "g",
            "--to",
            "mg"
        ]))
    );
    assert_eq!(
        cmd(tk!(["500g"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 500", "--unit", "g"]))
    );
    assert_eq!(
        cmd(tk!(["9999999.999", "ng"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 9999999.999",
            "--unit",
            "ng"
        ]))
    );
    assert_eq!(
        cmd(tk!(["9999999.999ng"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 9999999.999",
            "--unit",
            "ng"
        ]))
    );
    assert_eq!(
        cmd(tk!(["1000ounce"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 1000",
            "--unit",
            "ounce"
        ]))
    );
    assert_eq!(
        cmd(tk!(["1000oz"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 1000", "--unit", "oz"]))
    );
    assert_eq!(
        cmd(tk!(["2.5ft"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 2.5", "--unit", "ft"]))
    );
    assert_eq!(
        cmd(tk!(["2.5ft", "to", "nm"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 2.5",
            "--unit",
            "ft",
            "--to",
            "nm"
        ]))
    );
    assert_eq!(
        cmd(tk!(["10in", "to", "centimeter"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 10",
            "--unit",
            "in",
            "--to",
            "centimeter"
        ]))
    );
    assert_eq!(
        cmd(tk!(["10KiB"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " 10", "--unit", "KiB"]))
    );
    assert_eq!(
        cmd(tk!(["100bytes", "to", "tibibyte"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 100",
            "--unit",
            "bytes",
            "--to",
            "tibibyte"
        ]))
    );
    assert_eq!(
        cmd(tk!(["-100", "°F"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-unit", "--value", " -100", "--unit", "°F"]))
    );
    assert_eq!(
        cmd(tk!(["100.0", "m/s"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 100.0",
            "--unit",
            "m/s"
        ]))
    );
    assert_eq!(
        cmd(tk!(["1000mmHg"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 1000",
            "--unit",
            "mmHg"
        ]))
    );
    assert_eq!(
        cmd(tk!(["1000mmHg", "to", "bar"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-unit",
            "--value",
            " 1000",
            "--unit",
            "mmHg",
            "--to",
            "bar"
        ]))
    );
}
