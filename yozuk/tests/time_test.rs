#![cfg(all(feature = "modelgen", feature = "yozuk-skill-time"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn current_time() {
    assert_eq!(
        cmd(tk!(["now"])),
        CommandArgs::new().add_args(["yozuk-skill-time"])
    );
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
