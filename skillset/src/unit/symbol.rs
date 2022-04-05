use super::unit::UnitPrefix::*;
use super::unit::*;
use std::iter;

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

pub fn parse_symbol(s: &str) -> Option<(Option<UnitPrefix>, BaseUnit)> {
    ENTRIES
        .iter()
        .flat_map(|entry| {
            iter::once(None)
                .chain(entry.prefixes.iter().map(|prefix| Some(*prefix)))
                .map(|prefix| (prefix, entry.base))
        })
        .find(|(prefix, base)| {
            format!(
                "{}{}",
                prefix.map(|p| p.to_string()).unwrap_or_default(),
                base.to_string()
            ) == s
        })
}
