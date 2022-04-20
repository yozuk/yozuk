use super::conversion::*;
use super::entry::UnitPrefix::*;
use super::entry::*;
use bigdecimal::BigDecimal;
use lazy_static::lazy_static;
use strum::Display;

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    static ref KPH_MPS: BigDecimal = "3.6".parse().unwrap();
    static ref KPH_MPH: BigDecimal = "1.609344".parse().unwrap();
    static ref KPH_KNOT: BigDecimal = "1.852".parse().unwrap();
    static ref PASCAL_BAR: BigDecimal = "100000".parse().unwrap();
    static ref PASCAL_ATMOSPHERE: BigDecimal = "101325".parse().unwrap();
    static ref PASCAL_MMHG: BigDecimal = "133.322387415".parse().unwrap();
    static ref JOULE_CALORIE: BigDecimal = "4.1868".parse().unwrap();
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
    ConversionTable {
        base_unit: BaseUnit::KmsPerHour,
        base_prefixes: &[],
        entries: &[
            ConversionEntry {
                base_unit: BaseUnit::MsPerSec,
                base_prefixes: &[],
                convert_to_base: |value| value * KPH_MPS.clone(),
                convert_from_base: |value| value / KPH_MPS.clone(),
            },
            ConversionEntry {
                base_unit: BaseUnit::MilesPerHour,
                base_prefixes: &[],
                convert_to_base: |value| value * KPH_MPH.clone(),
                convert_from_base: |value| value / KPH_MPH.clone(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Knot,
                base_prefixes: &[],
                convert_to_base: |value| value * KPH_KNOT.clone(),
                convert_from_base: |value| value / KPH_KNOT.clone(),
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
                convert_to_base: |value| value * PASCAL_BAR.clone(),
                convert_from_base: |value| value / PASCAL_BAR.clone(),
            },
            ConversionEntry {
                base_unit: BaseUnit::Atmosphere,
                base_prefixes: &[],
                convert_to_base: |value| value * PASCAL_ATMOSPHERE.clone(),
                convert_from_base: |value| value / PASCAL_ATMOSPHERE.clone(),
            },
            ConversionEntry {
                base_unit: BaseUnit::MmHg,
                base_prefixes: &[],
                convert_to_base: |value| value * PASCAL_MMHG.clone(),
                convert_from_base: |value| value / PASCAL_MMHG.clone(),
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
            convert_to_base: |value| value * JOULE_CALORIE.clone(),
            convert_from_base: |value| value / JOULE_CALORIE.clone(),
        }],
    },
];
