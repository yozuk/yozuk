#![cfg(feature = "yozuk-skill-consts")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn pi() {
    assert_eq!(
        cmd(tk!(["pi"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "pi"]))
    );
    assert_eq!(
        cmd(tk!(["PI"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "pi"]))
    );
    assert_eq!(
        cmd(tk!(["π"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "pi"]))
    );
}

#[test]
fn speed_of_light() {
    assert_eq!(
        cmd(tk!(["Speed", "of", "Light"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "speed-of-light"]))
    );
    assert_eq!(
        cmd(tk!(["speed", "of", "light", "in", "vacuum"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "speed-of-light"]))
    );
}

#[test]
fn electron_mass() {
    assert_eq!(
        cmd(tk!(["Electron", "Mass"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "electron-mass"]))
    );
    assert_eq!(
        cmd(tk!(["mass", "of", "electron"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "electron-mass"]))
    );
}

#[test]
fn proton_mass() {
    assert_eq!(
        cmd(tk!(["Proton", "Mass"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "proton-mass"]))
    );
    assert_eq!(
        cmd(tk!(["mass", "of", "proton"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "proton-mass"]))
    );
}

#[test]
fn neutron_mass() {
    assert_eq!(
        cmd(tk!(["Neutron", "Mass"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "neutron-mass"]))
    );
    assert_eq!(
        cmd(tk!(["mass", "of", "neutron"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "neutron-mass"]))
    );
}

#[test]
fn muon_mass() {
    assert_eq!(
        cmd(tk!(["Muon", "Mass"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "muon-mass"]))
    );
    assert_eq!(
        cmd(tk!(["mass", "of", "muon"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "muon-mass"]))
    );
}

#[test]
fn tau_mass() {
    assert_eq!(
        cmd(tk!(["Tau", "Mass"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "tau-mass"]))
    );
    assert_eq!(
        cmd(tk!(["mass", "of", "tau"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "tau-mass"]))
    );
}

#[test]
fn top_quark_mass() {
    assert_eq!(
        cmd(tk!(["Top", "Quark", "Mass"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "top-quark-mass"]))
    );
    assert_eq!(
        cmd(tk!(["mass", "of", "top", "quark"])),
        Some(CommandArgs::new().add_args(["yozuk-skill-consts", "--name", "top-quark-mass"]))
    );
}
