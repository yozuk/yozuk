#![cfg(feature = "yozuk-skill-time")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn current_time() {
    assert_eq!(
        cmd(tk!(["current", "time"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-time"]))
    );
    assert_eq!(
        cmd(tk!(["time"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-time"]))
    );
    assert_eq!(
        cmd(tk!(["What", "time", "is", "it?"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-time"]))
    );
    assert_eq!(
        cmd(tk!(["What's", "the", "time?"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-time"]))
    );
}

#[test]
fn timestamp() {
    assert_eq!(
        cmd(tk!(["1640000000"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-time",
            "--timestamp",
            "1640000000000000000"
        ]))
    );
    assert_eq!(
        cmd(tk!(["1640000000000"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-time",
            "--timestamp",
            "1640000000000000000"
        ]))
    );
    assert_eq!(
        cmd(tk!(["1640000000000000000"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-time",
            "--timestamp",
            "1640000000000000000"
        ]))
    );
}

#[test]
fn rfc2822() {
    assert_eq!(
        cmd(tk!(["Sat, 12 Jun 1993 13:25:19 GMT"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-time",
            "--timestamp",
            "739891519000000000"
        ]))
    );
}

#[test]
fn conversion() {
    assert_eq!(
        cmd(tk!(["Sat, 12 Jun 1993 13:25:19 GMT", "to", "unix"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-time",
            "--timestamp",
            "739891519000000000",
            "--format",
            "unix"
        ]))
    );
    assert_eq!(
        cmd(tk!(["1640000000", "to", "3339"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-time",
            "--timestamp",
            "1640000000000000000",
            "--format",
            "3339"
        ]))
    );
    assert_eq!(
        cmd(tk!(["2019-07-14T00:00:00+00:00", "to", "TAI-64"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-time",
            "--timestamp",
            "1563062400000000000",
            "--format",
            "TAI-64"
        ]))
    );
}
