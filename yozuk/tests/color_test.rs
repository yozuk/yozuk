#![cfg(feature = "yozuk-skill-color")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn css_color() {
    assert_eq!(
        cmd(tk!(["#aaa"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-color", "#aaa"]))
    );
    assert_eq!(
        cmd(tk!(["DeepPink"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-color", "DeepPink"]))
    );
    assert_eq!(
        cmd(tk!(["rgb(0% 100% 0%)"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-color", "rgb(0% 100% 0%)"]))
    );
    assert_eq!(
        cmd(tk!(["rgba(1.0, 1.0, 1.0, 0.5)"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-color", "rgba(1.0, 1.0, 1.0, 0.5)"]))
    );
    assert_eq!(
        cmd(tk!(["hsl(235 100% 50% / .5)"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-color", "hsl(235 100% 50% / .5)"]))
    );
}

#[test]
fn color_convertion() {
    assert_eq!(
        cmd(tk!(["#aaa", "to", "RGB"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-color", "--space", "RGB", "#aaa"]))
    );
    assert_eq!(
        cmd(tk!(["DeepPink", "to", "hsla"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-color", "--space", "hsla", "DeepPink"]))
    );
    assert_eq!(
        cmd(tk!(["rgb(0% 100% 0%)", "to", "HSV"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-color",
            "--space",
            "HSV",
            "rgb(0% 100% 0%)"
        ]))
    );
    assert_eq!(
        cmd(tk!(["rgba(1.0, 1.0, 1.0, 0.5)", "to", "hwb"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-color",
            "--space",
            "hwb",
            "rgba(1.0, 1.0, 1.0, 0.5)"
        ]))
    );
    assert_eq!(
        cmd(tk!(["hsl(235 100% 50% / .5)", "to", "rgba"])),
        Some(CommandArgs::new().add_args([
            "yozuk-skill-color",
            "--space",
            "rgba",
            "hsl(235 100% 50% / .5)"
        ]))
    );
}
