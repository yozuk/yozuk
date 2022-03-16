#![cfg(all(feature = "modelgen", feature = "yozuk-skill-digest"))]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn digest() {
    assert_eq!(
        cmd(tk!(["md5"])),
        CommandArgs::new().add_args(["yozuk-skill-digest", "--algorithm", "md5"])
    );
    assert_eq!(
        cmd(tk!(["sha2", "md5"])),
        CommandArgs::new().add_args([
            "yozuk-skill-digest",
            "--algorithm",
            "sha2",
            "--algorithm",
            "md5"
        ])
    );
}
