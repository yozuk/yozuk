#![cfg(feature = "yozuk-skill-smalltalk")]

mod common;
use common::{cmd, YOZUK};
use yozuk_sdk::prelude::*;

#[test]
fn deep_thought() {
    assert_eq!(
        cmd(tk!(["life", "universe", "everything"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-smalltalk", "--life-universe-everything"]))
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
        Some(CommandArgs::new().add_args(["yozuk-skill-smalltalk", "--life-universe-everything"]))
    );
}

#[test]
fn empty() {
    assert_eq!(
        YOZUK.get_commands(&[], &[]).into_iter().next(),
        Some(CommandArgs::new().add_args(["yozuk-skill-smalltalk"]))
    );
}
