use super::entry::UnitPrefix::*;
use super::entry::*;
use bigdecimal::BigDecimal;
use lazy_static::lazy_static;
use num_bigint::BigInt;
use std::iter;

lazy_static! {
    static ref GRAM_OUNCE: BigDecimal = "28.349523125".parse().unwrap();
    static ref GRAM_POUND: BigDecimal = "453.59237".parse().unwrap();
}

const TABLES: &[ConversionTable] = &[ConversionTable {
    base_unit: BaseUnit::Gram,
    base_prefixes: &[Nano, Micro, Milli, Kilo],
    entries: &[
        ConversionEntry {
            base_unit: BaseUnit::Ounce,
            base_prefixes: &[],
            convert_to_base: |value| value * GRAM_OUNCE.clone(),
            convert_from_base: |value| value / GRAM_OUNCE.clone(),
        },
        ConversionEntry {
            base_unit: BaseUnit::Pound,
            base_prefixes: &[],
            convert_to_base: |value| value * GRAM_POUND.clone(),
            convert_from_base: |value| value / GRAM_POUND.clone(),
        },
    ],
}];

pub fn convert(value: BigDecimal, base: BaseUnit) -> Vec<Unit> {
    TABLES
        .iter()
        .flat_map(|table| table.convert(value.clone(), base))
        .collect()
}

fn convert_prefixes<'a>(
    value: BigDecimal,
    base: BaseUnit,
    prefixes: &'a [UnitPrefix],
) -> impl Iterator<Item = Unit> + 'a {
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
