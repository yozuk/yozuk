#![cfg(all(feature = "modelgen", feature = "yozuk-skill-lipsum"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn lipsum() {
    assert_eq!(
        cmd(tk!(["Lorem", "ipsum"])),
        CommandArgs::new().add_args(["yozuk-skill-lipsum"])
    );
    assert_eq!(
        cmd(tk!(["Lorem", "ipsum", "dolor", "sit", "amet,"])),
        CommandArgs::new().add_args(["yozuk-skill-lipsum"])
    );
    assert_eq!(
        cmd(tk!(["lipsum"])),
        CommandArgs::new().add_args(["yozuk-skill-lipsum"])
    );
}
