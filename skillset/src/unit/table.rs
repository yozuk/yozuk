use super::conversion::*;
use super::entry::UnitPrefix::*;
use super::entry::*;
use bigdecimal::BigDecimal;
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseUnit {
    Gram,
    Ounce,
    Pound,

    Meter,
    Inch,
    Foot,
    Yard,
    Mile,

    Byte,

    Celsius,
    Fahrenheit,
    Kelvin,
}

impl ToString for BaseUnit {
    fn to_string(&self) -> String {
        match self {
            Self::Gram => "g",
            Self::Ounce => "oz.",
            Self::Pound => "lb",
            Self::Meter => "m",
            Self::Inch => "in",
            Self::Foot => "ft",
            Self::Yard => "yd",
            Self::Mile => "mi.",
            Self::Byte => "B",
            Self::Celsius => "°C",
            Self::Fahrenheit => "°F",
            Self::Kelvin => "K",
        }
        .to_string()
    }
}

pub const ENTRIES: &[UnitEntry] = &[
    UnitEntry {
        symbols: &["g"],
        base: BaseUnit::Gram,
        prefixes: &[Nano, Micro, Milli, Kilo],
    },
    UnitEntry {
        symbols: &["oz.", "oz", "ounce"],
        base: BaseUnit::Ounce,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["lb"],
        base: BaseUnit::Pound,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["m"],
        base: BaseUnit::Meter,
        prefixes: &[Nano, Micro, Milli, Kilo],
    },
    UnitEntry {
        symbols: &["in"],
        base: BaseUnit::Inch,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["ft"],
        base: BaseUnit::Foot,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["yd"],
        base: BaseUnit::Yard,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["mi.", "mi"],
        base: BaseUnit::Mile,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["B", "byte", "bytes"],
        base: BaseUnit::Byte,
        prefixes: &[Kilo, Mega, Giga, Tera, Kibi, Mebi, Gibi, Tibi],
    },
    UnitEntry {
        symbols: &["°C"],
        base: BaseUnit::Celsius,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["°F"],
        base: BaseUnit::Fahrenheit,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["K"],
        base: BaseUnit::Kelvin,
        prefixes: &[],
    },
];

lazy_static! {
    static ref GRAM_OUNCE: BigDecimal = "28.349523125".parse().unwrap();
    static ref GRAM_POUND: BigDecimal = "453.59237".parse().unwrap();
    static ref METER_INCH: BigDecimal = "0.0254".parse().unwrap();
    static ref METER_FEET: BigDecimal = "0.3048".parse().unwrap();
    static ref METER_YARD: BigDecimal = "0.9144".parse().unwrap();
    static ref METER_MILE: BigDecimal = "1609.344".parse().unwrap();
    static ref KELVIN_CELSIUS: BigDecimal = "273.15".parse().unwrap();
    static ref KELVIN_FAHRENHEIT_0: BigDecimal = "1.8".parse().unwrap();
    static ref KELVIN_FAHRENHEIT_1: BigDecimal = "459.67".parse().unwrap();
}

pub const TABLES: &[ConversionTable] = &[
    ConversionTable {
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
    },
    ConversionTable {
        base_unit: BaseUnit::Meter,
        base_prefixes: &[Nano, Micro, Milli, Kilo],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::Inch,
                base_prefixes: &[],
                convert_to_base: |value| value * METER_INCH.clone(),
                convert_from_base: |value| value / METER_INCH.clone(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Foot,
                base_prefixes: &[],
                convert_to_base: |value| value * METER_FEET.clone(),
                convert_from_base: |value| value / METER_FEET.clone(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Yard,
                base_prefixes: &[],
                convert_to_base: |value| value * METER_YARD.clone(),
                convert_from_base: |value| value / METER_YARD.clone(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Mile,
                base_prefixes: &[],
                convert_to_base: |value| value * METER_MILE.clone(),
                convert_from_base: |value| value / METER_MILE.clone(),
            },
        ],
    },
    ConversionTable {
        base_unit: BaseUnit::Byte,
        base_prefixes: &[Kilo, Mega, Giga, Tera, Kibi, Mebi, Gibi, Tibi],
        entries: &[],
    },
    ConversionTable {
        base_unit: BaseUnit::Kelvin,
        base_prefixes: &[],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::Celsius,
                base_prefixes: &[],
                convert_to_base: |value| value + KELVIN_CELSIUS.clone(),
                convert_from_base: |value| value - KELVIN_CELSIUS.clone(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Fahrenheit,
                base_prefixes: &[],
                convert_to_base: |value| {
                    (value + KELVIN_FAHRENHEIT_1.clone()) / KELVIN_FAHRENHEIT_0.clone()
                },
                convert_from_base: |value| {
                    (value * KELVIN_FAHRENHEIT_0.clone()) - KELVIN_FAHRENHEIT_1.clone()
                },
            },
        ],
    },
];
