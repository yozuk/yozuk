use super::table::*;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;

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
        let decimal = self.value.to_string();
        let (int, frac) = if let Some((int, frac)) = decimal.split_once('.') {
            (int, Some(frac))
        } else {
            (decimal.as_str(), None)
        };
        let int = String::from_utf8(
            int.as_bytes()
                .rchunks(3)
                .flat_map(|chunks| {
                    let mut v = chunks.to_vec();
                    v.reverse();
                    v.push(b' ');
                    v
                })
                .rev()
                .collect(),
        )
        .unwrap();
        let value = format!(
            "{}{}{}",
            int.trim_start_matches(' '),
            frac.map(|_| ".").unwrap_or_default(),
            frac.unwrap_or_default()
        );
        format!(
            "{} {}{}",
            value,
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

impl ToString for UnitPrefix {
    fn to_string(&self) -> String {
        match self {
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
        }
        .to_string()
    }
}
