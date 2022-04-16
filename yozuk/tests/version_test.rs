#![cfg(all(feature = "modelgen", feature = "yozuk-skill-version"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn build_info() {
    assert_eq!(
        cmd(tk!(["version", "info"])),
        CommandArgs::new().add_args(["yozuk-skill-version", "--version-info"])
    );
}
