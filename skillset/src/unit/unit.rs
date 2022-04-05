use bigdecimal::BigDecimal;

#[derive(Debug, Copy, Clone)]
pub struct UnitEntry {
    pub symbols: &'static [&'static str],
    pub scale: i64,
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
pub enum BaseUnit {
    Hertz,
    Meter,
    Gram,
    Ounce,
    Pound,
}

impl ToString for BaseUnit {
    fn to_string(&self) -> String {
        match self {
            Self::Hertz => "Hz",
            Self::Meter => "m",
            Self::Gram => "g",
            Self::Ounce => "oz.",
            Self::Pound => "lb",
        }
        .to_string()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnitPrefix {
    Nano,
    Micro,
    Milli,
    Kilo,
    Mega,
    Giga,
    Tera,
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
            Self::Mega => 6,
            Self::Giga => 9,
            Self::Tera => 12,
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
            Self::Mega => "M",
            Self::Giga => "G",
            Self::Tera => "T",
        }
        .to_string()
    }
}
