#![cfg(feature = "yozuk-skill-ip")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn ipaddr() {
    assert_eq!(
        cmd(tk!(["127.0.0.1"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-ip", "127.0.0.1"]))
    );
    assert_eq!(
        cmd(tk!(["::1"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-ip", "::1"]))
    );
    assert_eq!(
        cmd(tk!(["0000:0000:0000:0000:0000:ffff:7f00:0001"])),
        Some(
            CommandArgs::new()
                .add_args(["yozuk-skill-ip", "0000:0000:0000:0000:0000:ffff:7f00:0001"])
        )
    );
}
