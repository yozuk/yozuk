#![cfg(feature = "yozuk-skill-prime")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn is_prime() {
    assert_eq!(
        cmd(tk!(["Is", "0", "prime?"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-prime", "--test", "0"]))
    );
    assert_eq!(
        cmd(tk!(["Is", "982451653", "a", "prime", "number"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-prime", "--test", "982451653"]))
    );
    assert_eq!(
        cmd(tk!([
            "is",
            "37975227936943673922808872755445627854565536638199",
            "prime"
        ])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-prime",
            "--test",
            "37975227936943673922808872755445627854565536638199"
        ]))
    );
    assert_eq!(
        cmd(tk!(["is", "37975227936943673922808872755449", "prime?"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-prime",
            "--test",
            "37975227936943673922808872755449"
        ]))
    );
}
