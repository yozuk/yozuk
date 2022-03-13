#![cfg(all(feature = "modelgen", feature = "yozuk-skill-smalltalk"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn deep_thought() {
    assert_eq!(
        cmd(tk!(["life", "universe", "evertyhing"])),
        CommandArgs::new().add_args(["yozuk-skill-smalltalk", "--life-universe-everything"])
    );
    assert_eq!(
        cmd(tk!([
            "The",
            "answer",
            "to",
            "life,",
            "the",
            "universe,",
            "and",
            "everything"
        ])),
        CommandArgs::new().add_args(["yozuk-skill-smalltalk", "--life-universe-everything"])
    );
}

#[test]
fn empty() {
    assert_eq!(
        cmd(vec![]),
        CommandArgs::new().add_args(["yozuk-skill-smalltalk"])
    );
}
