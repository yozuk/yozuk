#![cfg(all(feature = "modelgen", feature = "yozuk-skill-version"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn build_info() {
    assert_eq!(
        cmd(tk!(["build", "info"])),
        CommandArgs::new().add_args(["yozuk-skill-version", "--build-info"])
    );
}
