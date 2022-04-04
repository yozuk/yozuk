use super::entry::UnitPrefix::*;
use super::entry::*;
use bigdecimal::BigDecimal;
use lazy_static::lazy_static;
use num_bigint::BigInt;
use std::iter;

pub fn convert(value: &BigDecimal, base: BaseUnit) -> Vec<Unit> {
    match base {
        BaseUnit::Gram => convert_gram(value),
        BaseUnit::Ounce => convert_ounce(value),
        BaseUnit::Pound => convert_pound(value),
    }
}

lazy_static! {
    static ref GRAM_OUNCE: BigDecimal = "28.349523125".parse().unwrap();
    static ref GRAM_POUND: BigDecimal = "453.59237".parse().unwrap();
}

pub fn convert_gram(value: &BigDecimal) -> Vec<Unit> {
    vec![
        Unit {
            value: value.clone() / GRAM_OUNCE.clone(),
            base: BaseUnit::Ounce,
            prefix: None,
        },
        Unit {
            value: value.clone() / GRAM_POUND.clone(),
            base: BaseUnit::Pound,
            prefix: None,
        },
    ]
    .into_iter()
    .chain(convert_prefixes(
        value,
        BaseUnit::Gram,
        &[Nano, Micro, Milli, Kilo],
    ))
    .collect()
}

pub fn convert_ounce(value: &BigDecimal) -> Vec<Unit> {
    let gram = value.clone() * GRAM_OUNCE.clone();
    vec![Unit {
        value: gram.clone() / GRAM_POUND.clone(),
        base: BaseUnit::Pound,
        prefix: None,
    }]
    .into_iter()
    .chain(convert_prefixes(
        &gram,
        BaseUnit::Gram,
        &[Nano, Micro, Milli, Kilo],
    ))
    .collect()
}

pub fn convert_pound(value: &BigDecimal) -> Vec<Unit> {
    let gram = value.clone() * GRAM_POUND.clone();
    vec![Unit {
        value: gram.clone() / GRAM_OUNCE.clone(),
        base: BaseUnit::Ounce,
        prefix: None,
    }]
    .into_iter()
    .chain(convert_prefixes(
        &gram,
        BaseUnit::Gram,
        &[Nano, Micro, Milli, Kilo],
    ))
    .collect()
}

fn convert_prefixes<'a>(
    value: &'a BigDecimal,
    base: BaseUnit,
    prefixes: &'a [UnitPrefix],
) -> impl Iterator<Item = Unit> + 'a {
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
            value: value.clone(),
            base,
            prefix: None,
        }))
}
