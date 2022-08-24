use super::entry::*;
use super::table::*;
use bigdecimal::BigDecimal;
use std::iter;

pub fn convert(value: BigDecimal, base: BaseUnit) -> Vec<Unit> {
    TABLES
        .iter()
        .flat_map(|table| table.convert(value.clone(), base))
        .collect()
}

fn convert_prefixes(
    value: BigDecimal,
    base: BaseUnit,
    base_filter: UnitFilter,
    prefixes: &[(UnitPrefix, UnitFilter)],
) -> impl Iterator<Item = Unit> + '_ {
    let base_value = value.clone();
    prefixes
        .iter()
        .map(move |(prefix, filter)| {
            let value = value.clone();
            let scale = prefix.scale();
            let value = value / scale;
            Unit {
                prefix: Some(*prefix),
                value,
                base,
                filter: *filter,
            }
        })
        .chain(iter::once(Unit {
            value: base_value,
            base,
            prefix: None,
            filter: base_filter,
        }))
}

pub struct ConversionTable {
    pub base_unit: BaseUnit,
    pub base_filter: UnitFilter,
    pub base_prefixes: &'static [(UnitPrefix, UnitFilter)],
    pub entries: &'static [ConversionEntry],
}

impl ConversionTable {
    pub fn convert(&self, value: BigDecimal, base: BaseUnit) -> Vec<Unit> {
        let base_value = if base == self.base_unit {
            value
        } else if let Some(value) = self
            .entries
            .iter()
            .find(|entry| entry.base_unit == base)
            .map(|entry| (entry.convert_to_base)(value))
        {
            value
        } else {
            return vec![];
        };
        let value = base_value.clone();
        self.entries
            .iter()
            .flat_map(|entry| {
                let value = (entry.convert_from_base)(base_value.clone());
                convert_prefixes(
                    value,
                    entry.base_unit,
                    entry.base_filter,
                    entry.base_prefixes,
                )
            })
            .chain(convert_prefixes(
                value,
                self.base_unit,
                self.base_filter,
                self.base_prefixes,
            ))
            .collect()
    }
}

pub struct ConversionEntry {
    pub base_unit: BaseUnit,
    pub base_filter: UnitFilter,
    pub base_prefixes: &'static [(UnitPrefix, UnitFilter)],
    pub convert_to_base: fn(BigDecimal) -> BigDecimal,
    pub convert_from_base: fn(BigDecimal) -> BigDecimal,
}
