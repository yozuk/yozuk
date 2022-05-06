#![cfg(feature = "yozuk-skill-time")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn current_time() {
    assert_eq!(
        cmd(tk!(["current", "time"])),
        CommandArgs::new().add_args(["yozuk-skill-time"])
    );
    assert_eq!(
        cmd(tk!(["time"])),
        CommandArgs::new().add_args(["yozuk-skill-time"])
    );
    assert_eq!(
        cmd(tk!(["What", "time", "is", "it?"])),
        CommandArgs::new().add_args(["yozuk-skill-time"])
    );
    assert_eq!(
        cmd(tk!(["What's", "the", "time?"])),
        CommandArgs::new().add_args(["yozuk-skill-time"])
    );
}

#[test]
fn fuzzy_time() {
    assert_eq!(
        cmd(tk!(["now"])),
        CommandArgs::new().add_args(["yozuk-skill-time", "--exp", "now"])
    );
    assert_eq!(
        cmd(tk!(["five days after this friday"])),
        CommandArgs::new().add_args(["yozuk-skill-time", "--exp", "five days after this friday"])
    );
    assert_eq!(
        cmd(tk!(["Two days after 2/12/22 5:00 AM"])),
        CommandArgs::new().add_args([
            "yozuk-skill-time",
            "--exp",
            "Two days after 2/12/22 5:00 AM"
        ])
    );
}

#[test]
fn timestamp() {
    assert_eq!(
        cmd(tk!(["1640000000"])),
        CommandArgs::new().add_args(["yozuk-skill-time", "--timestamp", "1640000000000000000"])
    );
    assert_eq!(
        cmd(tk!(["1640000000000"])),
        CommandArgs::new().add_args(["yozuk-skill-time", "--timestamp", "1640000000000000000"])
    );
    assert_eq!(
        cmd(tk!(["1640000000000000000"])),
        CommandArgs::new().add_args(["yozuk-skill-time", "--timestamp", "1640000000000000000"])
    );
}
