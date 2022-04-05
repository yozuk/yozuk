use super::unit::UnitPrefix::*;
use super::unit::*;
use bigdecimal::{BigDecimal, FromPrimitive};
use num_bigint::BigInt;

pub fn convert(value: &BigDecimal, base: BaseUnit) -> Vec<Unit> {
    match base {
        BaseUnit::Gram => convert_gram(value, base).collect(),
        _ => vec![],
    }
}

pub fn convert_gram(value: &BigDecimal, base: BaseUnit) -> impl Iterator<Item = Unit> + '_ {
    vec![
        Unit {
            value: value.clone(),
            base: BaseUnit::Gram,
            prefix: None,
        },
        Unit {
            value: value.clone() / BigDecimal::from_f64(28.349523125).unwrap(),
            base: BaseUnit::Ounce,
            prefix: None,
        },
    ]
    .into_iter()
    .chain(convert_prefixes(value, base, &[Nano, Micro, Milli, Kilo]))
}

fn convert_prefixes<'a>(
    value: &'a BigDecimal,
    base: BaseUnit,
    prefixes: &'a [UnitPrefix],
) -> impl Iterator<Item = Unit> + 'a {
    prefixes.iter().map(move |prefix| {
        let value = value.clone();
        let scale = BigDecimal::new(BigInt::from(1), prefix.scale());
        Unit {
            prefix: Some(*prefix),
            value: (value * scale),
            base,
        }
    })
}
