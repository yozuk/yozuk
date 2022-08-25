#![cfg(feature = "yozuk-skill-chitchat")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn deep_thought() {
    assert_eq!(
        cmd(tk!(["life", "universe", "everything"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-chitchat", "--name", "42"]))
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
        Some(CommandArgs::new().add_args(["yozuk-skill-chitchat", "--name", "42"]))
    );
}

#[test]
fn early_bird() {
    assert_eq!(
        cmd(tk!(["Do", "you", "wake", "up", "early?"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-chitchat", "--name", "early-bird"]))
    );
    assert_eq!(
        cmd(tk!(["are", "you", "an", "early", "bird"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-chitchat", "--name", "early-bird"]))
    );
}

#[test]
fn help() {
    assert_eq!(
        cmd(tk!(["Help"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-chitchat", "--name", "help"]))
    );
    assert_eq!(
        cmd(tk!(["docs"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-chitchat", "--name", "help"]))
    );
}

#[test]
fn thanks() {
    assert_eq!(
        cmd(tk!(["thanks"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-chitchat", "--name", "thanks"]))
    );
    assert_eq!(
        cmd(tk!(["Thank", "you!"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-chitchat", "--name", "thanks"]))
    );
}
