#![cfg(all(feature = "modelgen", feature = "yozuk-skill-geo"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn open_location_code() {
    assert_eq!(
        cmd(tk!(["6PH57VP3+PR6"])),
        CommandArgs::new().add_args(["yozuk-skill-geo", "--olc", "6PH57VP3+PR6"])
    );
}
