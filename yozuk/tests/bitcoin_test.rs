#![cfg(feature = "yozuk-skill-bitcoin")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn test_address_generation() {
    assert_eq!(
        cmd(tk!(["BTC"])),
        CommandArgs::new().add_args(["yozuk-skill-bitcoin"])
    );
    assert_eq!(
        cmd(tk!(["bitcoin"])),
        CommandArgs::new().add_args(["yozuk-skill-bitcoin"])
    );
    assert_eq!(
        cmd(tk!(["generate", "bitcoin", "address"])),
        CommandArgs::new().add_args(["yozuk-skill-bitcoin"])
    );
    assert_eq!(
        cmd(tk!(["New", "bitcoin", "address"])),
        CommandArgs::new().add_args(["yozuk-skill-bitcoin"])
    );
}
