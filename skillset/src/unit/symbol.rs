use super::unit::UnitPrefix::*;
use super::unit::*;

pub const ENTRIES: &[UnitEntry] = &[
    UnitEntry {
        symbols: &["m"],
        scale: 0,
        base: BaseUnit::Meter,
        prefixes: &[Kilo],
    },
    UnitEntry {
        symbols: &["Hz", "㎐"],
        scale: 0,
        base: BaseUnit::Hertz,
        prefixes: &[Kilo, Mega, Giga, Tera],
    },
    UnitEntry {
        symbols: &["g"],
        scale: 0,
        base: BaseUnit::Gram,
        prefixes: &[Nano, Micro, Milli, Kilo],
    },
];
