#![cfg(feature = "yozuk-skill-lipsum")]

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
    assert_eq!(
        cmd(tk!(["generate", "dummy", "text"])),
        CommandArgs::new().add_args(["yozuk-skill-lipsum"])
    );
    assert_eq!(
        cmd(tk!(["dummy", "text"])),
        CommandArgs::new().add_args(["yozuk-skill-lipsum"])
    );
}

#[test]
fn lipsum_with_words() {
    assert_eq!(
        cmd(tk!(["Lorem", "ipsum", "300", "words"])),
        CommandArgs::new().add_args(["yozuk-skill-lipsum", "-n", "300"])
    );
    assert_eq!(
        cmd(tk!(["Lorem", "ipsum", "dolor", "sit", "amet,", "100"])),
        CommandArgs::new().add_args(["yozuk-skill-lipsum", "-n", "100"])
    );
    assert_eq!(
        cmd(tk!(["lipsum", "30"])),
        CommandArgs::new().add_args(["yozuk-skill-lipsum", "-n", "30"])
    );
    assert_eq!(
        cmd(tk!(["Generate", "300", "words", "dummy", "text"])),
        CommandArgs::new().add_args(["yozuk-skill-lipsum", "-n", "300"])
    );
    assert_eq!(
        cmd(tk!(["Dummy", "text", "300", "words"])),
        CommandArgs::new().add_args(["yozuk-skill-lipsum", "-n", "300"])
    );
}
