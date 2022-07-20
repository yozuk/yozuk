use super::table::*;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::fmt;
use thousands::Separable;

#[derive(Debug, Copy, Clone)]
pub struct UnitEntry {
    pub symbols: &'static [&'static str],
    pub base: BaseUnit,
    pub prefixes: &'static [UnitPrefix],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    pub value: BigDecimal,
    pub base: BaseUnit,
    pub prefix: Option<UnitPrefix>,
}

impl Unit {
    pub fn normalized(&self) -> Self {
        Self {
            value: self.value.with_prec(10).normalized(),
            ..self.clone()
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.value.to_string().separate_with_commas();
        write!(
            f,
            "{} {}{}",
            value,
            self.prefix
                .map(|prefix| prefix.to_string())
                .unwrap_or_default(),
            self.base
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnitPrefix {
    Nano,
    Micro,
    Milli,
    Hecto,
    Kilo,
    Mega,
    Giga,
    Tera,
    Kibi,
    Mebi,
    Gibi,
    Tibi,
}

impl UnitPrefix {
    pub fn scale(&self) -> BigDecimal {
        match self {
            Self::Nano => BigDecimal::new(BigInt::from(1), 9),
            Self::Micro => BigDecimal::new(BigInt::from(1), 6),
            Self::Milli => BigDecimal::new(BigInt::from(1), 3),
            Self::Hecto => BigDecimal::new(BigInt::from(1), -2),
            Self::Kilo => BigDecimal::new(BigInt::from(1), -3),
            Self::Mega => BigDecimal::new(BigInt::from(1), -6),
            Self::Giga => BigDecimal::new(BigInt::from(1), -9),
            Self::Tera => BigDecimal::new(BigInt::from(1), -12),
            Self::Kibi => BigDecimal::new(BigInt::from(1024), 0),
            Self::Mebi => Self::Kibi.scale() * Self::Kibi.scale(),
            Self::Gibi => Self::Mebi.scale() * Self::Kibi.scale(),
            Self::Tibi => Self::Gibi.scale() * Self::Kibi.scale(),
        }
    }
}

impl fmt::Display for UnitPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Nano => "n",
            Self::Micro => "µ",
            Self::Milli => "m",
            Self::Hecto => "h",
            Self::Kilo => "k",
            Self::Mega => "M",
            Self::Giga => "G",
            Self::Tera => "T",
            Self::Kibi => "Ki",
            Self::Gibi => "Gi",
            Self::Mebi => "Mi",
            Self::Tibi => "Ti",
        };
        write!(f, "{}", s)
    }
}
