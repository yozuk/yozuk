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
