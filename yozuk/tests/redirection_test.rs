mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn test_redirection() {
    assert_eq!(
        cmd(tk!(["test", "command", "redirect"])),
        Some(CommandArgs::new().add_args(["yozuk-redirect", "test", "redirect"]))
    );
    assert_eq!(
        cmd(tk!(["Test", "commands", "redirect"])),
        Some(CommandArgs::new().add_args(["yozuk-redirect", "test", "redirect"]))
    );
}
