#![cfg(feature = "yozuk-skill-urlencode")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn urlencode() {
    assert_eq!(
        cmd(tk!(["Hi%21%20I%27m%20Yozuk."])),
        CommandArgs::new().add_args(["yozuk-skill-urlencode", "Hi%21%20I%27m%20Yozuk."])
    );
}
