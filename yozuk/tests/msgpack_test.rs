#![cfg(feature = "yozuk-skill-msgpack")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn decode() {
    assert_eq!(
        cmd(tk!(["3wAAAAOjc3Vio2FiY6RuYW1lpVlvenVro2lhdM5aX/iu"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-msgpack",
            "3wAAAAOjc3Vio2FiY6RuYW1lpVlvenVro2lhdM5aX/iu"
        ]))
    );
}
