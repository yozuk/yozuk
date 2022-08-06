#![cfg(feature = "yozuk-skill-smalltalk")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn deep_thought() {
    assert_eq!(
        cmd(tk!(["life", "universe", "everything"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-smalltalk", "--name", "42"]))
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
        Some(CommandArgs::new().add_args(["yozuk-skill-smalltalk", "--name", "42"]))
    );
}

#[test]
fn early_bird() {
    assert_eq!(
        cmd(tk!(["Do", "you", "wake", "up", "early?"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-smalltalk", "--name", "early-bird"]))
    );
    assert_eq!(
        cmd(tk!(["are", "you", "an", "early", "bird"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-smalltalk", "--name", "early-bird"]))
    );
}

#[test]
fn help() {
    assert_eq!(
        cmd(tk!(["Help"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-smalltalk", "--name", "help"]))
    );
    assert_eq!(
        cmd(tk!(["docs"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-smalltalk", "--name", "help"]))
    );
}
