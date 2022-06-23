#![cfg(feature = "yozuk-skill-nanoid")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn single_nanoid() {
    assert_eq!(
        cmd(tk!(["nanoid"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-nanoid",
            "-n",
            "1",
            "-c",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_~"
        ]))
    );
    assert_eq!(
        cmd(tk!(["1", "NanoID"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-nanoid",
            "-n",
            "1",
            "-c",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_~"
        ]))
    );
    assert_eq!(
        cmd(tk!(["NanoID", "uppercase"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-nanoid",
            "-n",
            "1",
            "-c",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        ]))
    );
    assert_eq!(
        cmd(tk!(["NanoID", "number"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-nanoid", "-n", "1", "-c", "0123456789"]))
    );
    assert_eq!(
        cmd(tk!(["NanoID", "lower", "alphabet"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-nanoid",
            "-n",
            "1",
            "-c",
            "abcdefghijklmnopqrstuvwxyz"
        ]))
    );
}

#[test]
fn multi_nanoid() {
    assert_eq!(
        cmd(tk!(["4", "nanoid"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-nanoid",
            "-n",
            "4",
            "-c",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_~"
        ]))
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "NanoID"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-nanoid",
            "-n",
            "10",
            "-c",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_~"
        ]))
    );
    assert_eq!(
        cmd(tk!(["generate", "999", "NanoIDs"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-nanoid",
            "-n",
            "999",
            "-c",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_~"
        ]))
    );
    assert_eq!(
        cmd(tk!(["generate", "10", "nanoid."])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-nanoid",
            "-n",
            "10",
            "-c",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_~"
        ]))
    );
    assert_eq!(
        cmd(tk!(["Hi", "yozuk,", "generate", "10", "nanoids."])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-nanoid",
            "-n",
            "10",
            "-c",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_~"
        ]))
    );
}
