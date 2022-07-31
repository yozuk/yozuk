use super::conversion::*;
use super::entry::UnitPrefix::*;
use super::entry::*;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use strum::Display;

#[derive(Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseUnit {
    #[strum(serialize = "g")]
    Gram,
    #[strum(serialize = "oz.")]
    Ounce,
    #[strum(serialize = "lb")]
    Pound,

    #[strum(serialize = "m")]
    Meter,
    #[strum(serialize = "in")]
    Inch,
    #[strum(serialize = "ft")]
    Foot,
    #[strum(serialize = "yd")]
    Yard,
    #[strum(serialize = "mi.")]
    Mile,

    #[strum(serialize = "B")]
    Byte,

    #[strum(serialize = "°C")]
    Celsius,
    #[strum(serialize = "°F")]
    Fahrenheit,
    #[strum(serialize = "K")]
    Kelvin,

    #[strum(serialize = "km/h")]
    KmsPerHour,
    #[strum(serialize = "m/s")]
    MsPerSec,
    #[strum(serialize = "mph")]
    MilesPerHour,
    #[strum(serialize = "kn.")]
    Knot,

    #[strum(serialize = "Pa")]
    Pascal,
    #[strum(serialize = "bar")]
    Bar,
    #[strum(serialize = "atm")]
    Atmosphere,
    #[strum(serialize = "mmHg")]
    MmHg,

    #[strum(serialize = "Hz")]
    Hertz,

    #[strum(serialize = "J")]
    Joule,
    #[strum(serialize = "Cal")]
    Calorie,
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
    UnitEntry {
        symbols: &["km/h", "kph"],
        base: BaseUnit::KmsPerHour,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["m/s"],
        base: BaseUnit::MsPerSec,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["mph", "mi/h"],
        base: BaseUnit::MilesPerHour,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["kn.", "kn", "kt"],
        base: BaseUnit::Knot,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["Pa"],
        base: BaseUnit::Pascal,
        prefixes: &[Hecto],
    },
    UnitEntry {
        symbols: &["bar"],
        base: BaseUnit::Bar,
        prefixes: &[Milli],
    },
    UnitEntry {
        symbols: &["atm"],
        base: BaseUnit::Atmosphere,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["mmHg", "mmhg"],
        base: BaseUnit::MmHg,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["Hz", "hz"],
        base: BaseUnit::Hertz,
        prefixes: &[Kilo, Mega, Giga, Tera],
    },
    UnitEntry {
        symbols: &["J"],
        base: BaseUnit::Joule,
        prefixes: &[Kilo],
    },
    UnitEntry {
        symbols: &["Cal", "cal"],
        base: BaseUnit::Calorie,
        prefixes: &[Kilo],
    },
];

pub const TABLES: &[ConversionTable] = &[
    ConversionTable {
        base_unit: BaseUnit::Gram,
        base_prefixes: &[Nano, Micro, Milli, Kilo],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::Ounce,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("28.349523125").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("28.349523125").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Pound,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("453.59237").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("453.59237").unwrap(),
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
                convert_to_base: |value| value * BigDecimal::from_str("0.0254").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("0.0254").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Foot,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("0.3048").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("0.3048").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Yard,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("0.9144").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("0.9144").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Mile,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("1609.344").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("1609.344").unwrap(),
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
                convert_to_base: |value| value + BigDecimal::from_str("273.15").unwrap(),
                convert_from_base: |value| value - BigDecimal::from_str("273.15").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Fahrenheit,
                base_prefixes: &[],
                convert_to_base: |value| {
                    (value + BigDecimal::from_str("459.67").unwrap())
                        / BigDecimal::from_str("1.8").unwrap()
                },
                convert_from_base: |value| {
                    (value * BigDecimal::from_str("1.8").unwrap())
                        - BigDecimal::from_str("459.67").unwrap()
                },
            },
        ],
    },
    ConversionTable {
        base_unit: BaseUnit::KmsPerHour,
        base_prefixes: &[],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::MsPerSec,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("3.6").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("3.6").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::MilesPerHour,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("1.609344").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("1.609344").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Knot,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("1.852").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("1.852").unwrap(),
            },
        ],
    },
    ConversionTable {
        base_unit: BaseUnit::Pascal,
        base_prefixes: &[Hecto],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::Bar,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("100000").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("100000").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Atmosphere,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("101325").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("101325").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::MmHg,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("133.322387415").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("133.322387415").unwrap(),
            },
        ],
    },
    ConversionTable {
        base_unit: BaseUnit::Hertz,
        base_prefixes: &[Kilo, Mega, Giga],
        entries: &[],
    },
    ConversionTable {
        base_unit: BaseUnit::Joule,
        base_prefixes: &[Kilo],
        entries: &[ConversionEntry {
            base_unit: BaseUnit::Calorie,
            base_prefixes: &[Kilo],
            convert_to_base: |value| value * BigDecimal::from_str("4.1868").unwrap(),
            convert_from_base: |value| value / BigDecimal::from_str("4.1868").unwrap(),
        }],
    },
];
