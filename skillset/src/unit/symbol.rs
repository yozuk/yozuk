use super::entry::*;
use super::table::*;
use bigdecimal::BigDecimal;
use std::iter;
use std::str::FromStr;

fn symbols() -> impl Iterator<Item = (Option<UnitPrefix>, BaseUnit, String)> {
    ENTRIES.iter().flat_map(|entry| {
        iter::once(None)
            .chain(entry.prefixes.iter().map(|prefix| Some(*prefix)))
            .flat_map(move |prefix| {
                entry.symbols.iter().map(move |sym| {
                    (
                        prefix,
                        entry.base,
                        format!(
                            "{}{}",
                            prefix.map(|p| p.to_string()).unwrap_or_default(),
                            sym
                        ),
                    )
                })
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
