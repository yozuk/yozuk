use super::unit::*;
use bigdecimal::{BigDecimal, FromPrimitive};

pub fn convert_gram(value: &Unit) -> Vec<Unit> {
    vec![Unit {
        value: value.value.clone() / BigDecimal::from_f64(28.349523125).unwrap(),
        base: BaseUnit::Ounce,
        prefix: None,
    }]
}
