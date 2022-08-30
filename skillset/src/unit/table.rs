use super::conversion::*;
use super::entry::UnitPrefix::*;
use super::entry::*;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use strum_macros::Display;

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
    #[strum(serialize = "ly")]
    LightYear,
    #[strum(serialize = "pc")]
    Parsec,

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
        symbols: &["g", "gram"],
        base: BaseUnit::Gram,
        prefixes: &[Nano, Micro, Milli, Kilo],
    },
    UnitEntry {
        symbols: &["oz.", "oz", "ounce"],
        base: BaseUnit::Ounce,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["lb", "lbs", "pound", "pounds"],
        base: BaseUnit::Pound,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["m", "meter"],
        base: BaseUnit::Meter,
        prefixes: &[Nano, Micro, Milli, Centi, Kilo],
    },
    UnitEntry {
        symbols: &["in", "inch"],
        base: BaseUnit::Inch,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["ft", "feet", "foot"],
        base: BaseUnit::Foot,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["yd", "yard"],
        base: BaseUnit::Yard,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["mi.", "mi", "mile"],
        base: BaseUnit::Mile,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["ly", "light-year", "lightyear"],
        base: BaseUnit::LightYear,
        prefixes: &[Kilo, Mega, Giga],
    },
    UnitEntry {
        symbols: &["pc", "parsec"],
        base: BaseUnit::Parsec,
        prefixes: &[Kilo, Mega, Giga],
    },
    UnitEntry {
        symbols: &["B", "byte", "bytes"],
        base: BaseUnit::Byte,
        prefixes: &[Kilo, Mega, Giga, Tera, Kibi, Mebi, Gibi, Tibi],
    },
    UnitEntry {
        symbols: &["°C", "c", "celcius"],
        base: BaseUnit::Celsius,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["°F", "f", "fahrenheit"],
        base: BaseUnit::Fahrenheit,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["K", "Kelvin"],
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
        symbols: &["kn.", "kn", "kt", "knot"],
        base: BaseUnit::Knot,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["Pa", "pascal"],
        base: BaseUnit::Pascal,
        prefixes: &[Hecto],
    },
    UnitEntry {
        symbols: &["bar"],
        base: BaseUnit::Bar,
        prefixes: &[Milli],
    },
    UnitEntry {
        symbols: &["atm", "atmosphere"],
        base: BaseUnit::Atmosphere,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["mmHg", "mmhg"],
        base: BaseUnit::MmHg,
        prefixes: &[],
    },
    UnitEntry {
        symbols: &["Hz", "hz", "heartz"],
        base: BaseUnit::Hertz,
        prefixes: &[Kilo, Mega, Giga, Tera],
    },
    UnitEntry {
        symbols: &["J", "joule"],
        base: BaseUnit::Joule,
        prefixes: &[Kilo],
    },
    UnitEntry {
        symbols: &["Cal", "cal", "calorie"],
        base: BaseUnit::Calorie,
        prefixes: &[Kilo],
    },
];

pub const TABLES: &[ConversionTable] = &[
    ConversionTable {
        base_unit: BaseUnit::Gram,
        base_filter: UnitFilter::MaximumScale(5),
        base_prefixes: &[
            (Nano, UnitFilter::MaximumScale(5)),
            (Micro, UnitFilter::MaximumScale(5)),
            (Milli, UnitFilter::MaximumScale(5)),
            (Kilo, UnitFilter::MaximumScale(5)),
        ],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::Ounce,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("28.349523125").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("28.349523125").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Pound,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("453.59237").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("453.59237").unwrap(),
            },
        ],
    },
    ConversionTable {
        base_unit: BaseUnit::Meter,
        base_filter: UnitFilter::MaximumScale(5),
        base_prefixes: &[
            (Nano, UnitFilter::MaximumScale(5)),
            (Micro, UnitFilter::MaximumScale(5)),
            (Milli, UnitFilter::MaximumScale(5)),
            (Centi, UnitFilter::MaximumScale(5)),
            (Kilo, UnitFilter::MaximumScale(5)),
        ],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::Inch,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("0.0254").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("0.0254").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Foot,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("0.3048").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("0.3048").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Yard,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("0.9144").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("0.9144").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Mile,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("1609.344").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("1609.344").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::LightYear,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[
                    (Kilo, UnitFilter::MaximumScale(5)),
                    (Mega, UnitFilter::MaximumScale(5)),
                    (Giga, UnitFilter::MaximumScale(5)),
                ],
                convert_to_base: |value| value * BigDecimal::from_str("9460730472580800").unwrap(),
                convert_from_base: |value| {
                    value / BigDecimal::from_str("9460730472580800").unwrap()
                },
            },
            ConversionEntry {
                base_unit: BaseUnit::Parsec,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[
                    (Kilo, UnitFilter::MaximumScale(5)),
                    (Mega, UnitFilter::MaximumScale(5)),
                    (Giga, UnitFilter::MaximumScale(5)),
                ],
                convert_to_base: |value| value * BigDecimal::from_str("30856775814913673").unwrap(),
                convert_from_base: |value| {
                    value / BigDecimal::from_str("30856775814913673").unwrap()
                },
            },
        ],
    },
    ConversionTable {
        base_unit: BaseUnit::Byte,
        base_filter: UnitFilter::Always,
        base_prefixes: &[
            (Kilo, UnitFilter::MaximumScale(5)),
            (Mega, UnitFilter::MaximumScale(5)),
            (Giga, UnitFilter::MaximumScale(5)),
            (Tera, UnitFilter::MaximumScale(5)),
            (Kibi, UnitFilter::MaximumScale(5)),
            (Mebi, UnitFilter::MaximumScale(5)),
            (Gibi, UnitFilter::MaximumScale(5)),
            (Tibi, UnitFilter::MaximumScale(5)),
        ],
        entries: &[],
    },
    ConversionTable {
        base_unit: BaseUnit::Kelvin,
        base_filter: UnitFilter::Always,
        base_prefixes: &[],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::Celsius,
                base_filter: UnitFilter::Always,
                base_prefixes: &[],
                convert_to_base: |value| value + BigDecimal::from_str("273.15").unwrap(),
                convert_from_base: |value| value - BigDecimal::from_str("273.15").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Fahrenheit,
                base_filter: UnitFilter::Always,
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
        base_filter: UnitFilter::MaximumScale(5),
        base_prefixes: &[],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::MsPerSec,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("3.6").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("3.6").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::MilesPerHour,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("1.609344").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("1.609344").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Knot,
                base_filter: UnitFilter::Optional,
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("1.852").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("1.852").unwrap(),
            },
        ],
    },
    ConversionTable {
        base_unit: BaseUnit::Pascal,
        base_filter: UnitFilter::MaximumScale(5),
        base_prefixes: &[(Hecto, UnitFilter::MaximumScale(5))],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::Bar,
                base_filter: UnitFilter::Optional,
                base_prefixes: &[(Milli, UnitFilter::Optional)],
                convert_to_base: |value| value * BigDecimal::from_str("100000").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("100000").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Atmosphere,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("101325").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("101325").unwrap(),
            },
            ConversionEntry {
                base_unit: BaseUnit::MmHg,
                base_filter: UnitFilter::MaximumScale(5),
                base_prefixes: &[],
                convert_to_base: |value| value * BigDecimal::from_str("133.322387415").unwrap(),
                convert_from_base: |value| value / BigDecimal::from_str("133.322387415").unwrap(),
            },
        ],
    },
    ConversionTable {
        base_unit: BaseUnit::Hertz,
        base_filter: UnitFilter::MaximumScale(5),
        base_prefixes: &[
            (Kilo, UnitFilter::MaximumScale(5)),
            (Mega, UnitFilter::MaximumScale(5)),
            (Giga, UnitFilter::MaximumScale(5)),
            (Tera, UnitFilter::MaximumScale(5)),
        ],
        entries: &[],
    },
    ConversionTable {
        base_unit: BaseUnit::Joule,
        base_filter: UnitFilter::MaximumScale(5),
        base_prefixes: &[(Kilo, UnitFilter::MaximumScale(5))],
        entries: &[ConversionEntry {
            base_unit: BaseUnit::Calorie,
            base_filter: UnitFilter::MaximumScale(5),
            base_prefixes: &[(Kilo, UnitFilter::MaximumScale(5))],
            convert_to_base: |value| value * BigDecimal::from_str("4.1868").unwrap(),
            convert_from_base: |value| value / BigDecimal::from_str("4.1868").unwrap(),
        }],
    },
];
