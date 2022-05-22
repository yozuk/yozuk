#![cfg(feature = "yozuk-skill-bech32")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn bech32() {
    assert_eq!(
        cmd(tk!(["tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx"])),
        CommandArgs::new().add_args([
            "yozuk-skill-bech32",
            "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx"
        ])
    );
}

#[test]
fn bech32m() {
    assert_eq!(
        cmd(tk!(["abcdef1l7aum6echk45nj3s0wdvt2fg8x9yrzpqzd3ryx"])),
        CommandArgs::new().add_args([
            "yozuk-skill-bech32",
            "abcdef1l7aum6echk45nj3s0wdvt2fg8x9yrzpqzd3ryx"
        ])
    );
}
