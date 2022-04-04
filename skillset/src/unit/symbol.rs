use super::entry::UnitPrefix::*;
use super::entry::*;
use bigdecimal::BigDecimal;
use std::iter;
use std::str::FromStr;

pub const ENTRIES: &[UnitEntry] = &[
    UnitEntry {
        symbols: &["g"],
        scale: 0,
        base: BaseUnit::Gram,
        prefixes: &[Nano, Micro, Milli, Kilo],
    },
    UnitEntry {
        symbols: &["oz.", "oz", "ounce"],
        scale: 0,
        base: BaseUnit::Ounce,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["lb"],
        scale: 0,
        base: BaseUnit::Pound,
        prefixes: &[],
    },
];

fn symbols() -> impl Iterator<Item = (Option<UnitPrefix>, BaseUnit, String)> {
    ENTRIES.iter().flat_map(|entry| {
        iter::once(None)
            .chain(entry.prefixes.iter().map(|prefix| Some(*prefix)))
            .map(|prefix| {
                (
                    prefix,
                    entry.base,
                    format!(
                        "{}{}",
                        prefix.map(|p| p.to_string()).unwrap_or_default(),
                        entry.base.to_string()
                    ),
                )
            })
    })
}

pub fn parse_symbol(s: &str) -> Option<(Option<UnitPrefix>, BaseUnit)> {
    symbols()
        .find(|(_, _, sym)| sym == s)
        .map(|(prefix, base, _)| (prefix, base))
}

pub fn parse_num_symbol(s: &str) -> Option<(&str, &str)> {
    symbols()
        .find(|(_, _, sym)| {
            s.strip_suffix(sym.as_str())
                .and_then(|s| BigDecimal::from_str(s).ok())
                .is_some()
        })
        .map(|(_, _, sym)| {
            let len = s.len() - sym.len();
            (&s[..len], &s[len..])
        })
}
