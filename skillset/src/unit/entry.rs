use super::table::*;
use bigdecimal::BigDecimal;

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

impl ToString for Unit {
    fn to_string(&self) -> String {
        format!(
            "{} {}{}",
            self.value,
            self.prefix
                .map(|prefix| prefix.to_string())
                .unwrap_or_default(),
            self.base.to_string()
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnitPrefix {
    Nano,
    Micro,
    Milli,
    Kilo,
}

pub trait Scale {
    fn scale(&self) -> i64;
}

impl Scale for UnitPrefix {
    fn scale(&self) -> i64 {
        match self {
            Self::Nano => -9,
            Self::Micro => -6,
            Self::Milli => -3,
            Self::Kilo => 3,
        }
    }
}

impl ToString for UnitPrefix {
    fn to_string(&self) -> String {
        match self {
            Self::Nano => "n",
            Self::Micro => "µ",
            Self::Milli => "m",
            Self::Kilo => "k",
        }
        .to_string()
    }
}
