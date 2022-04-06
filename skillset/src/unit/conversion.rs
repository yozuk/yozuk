use super::entry::*;
use super::table::*;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
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
    prefixes: &[UnitPrefix],
) -> impl Iterator<Item = Unit> + '_ {
    let base_value = value.clone();
    prefixes
        .iter()
        .map(move |prefix| {
            let value = value.clone();
            let scale = BigDecimal::new(BigInt::from(1), prefix.scale());
            Unit {
                prefix: Some(*prefix),
                value: (value * scale),
                base,
            }
        })
        .chain(iter::once(Unit {
            value: base_value,
            base,
            prefix: None,
        }))
}

pub struct ConversionTable {
    pub base_unit: BaseUnit,
    pub base_prefixes: &'static [UnitPrefix],
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
                convert_prefixes(value, entry.base_unit, entry.base_prefixes)
            })
            .chain(convert_prefixes(value, self.base_unit, self.base_prefixes))
            .collect()
    }
}

pub struct ConversionEntry {
    pub base_unit: BaseUnit,
    pub base_prefixes: &'static [UnitPrefix],
    pub convert_to_base: fn(BigDecimal) -> BigDecimal,
    pub convert_from_base: fn(BigDecimal) -> BigDecimal,
}
