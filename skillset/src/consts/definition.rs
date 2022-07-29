use super::Constant;
use itertools::iproduct;
use phf::phf_map;
use yozuk_sdk::prelude::*;

pub static DEFINITIONS: phf::Map<&'static str, Constant> = phf_map! {
    "u8-max" => Constant {
        name: "The largest value of 16-bit unsigned integer",
        tokens:
            || iproduct!(["u8", "uint8", "uint8_t"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:u8-max",
                    key; "keyword:u8-max"
                ])
            }).collect(),
        value: "255",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "u8-min" => Constant {
        name: "The smallest value of 16-bit unsigned integer",
        tokens:
            || iproduct!(["u8", "uint8", "uint8_t"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:u8-min",
                    key; "keyword:u8-min"
                ])
            }).collect(),
        value: "0",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "u16-max" => Constant {
        name: "The largest value of 16-bit unsigned integer",
        tokens:
            || iproduct!(["u16", "uint16", "uint16_t"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:u16-max",
                    key; "keyword:u16-max"
                ])
            }).collect(),
        value: "65535",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "u16-min" => Constant {
        name: "The smallest value of 16-bit unsigned integer",
        tokens:
            || iproduct!(["u16", "uint16", "uint16_t"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:u16-min",
                    key; "keyword:u16-min"
                ])
            }).collect(),
        value: "0",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "u32-max" => Constant {
        name: "The largest value of 32-bit unsigned integer",
        tokens:
            || iproduct!(["u32", "uint32", "uint32_t"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:u32-max",
                    key; "keyword:u32-max"
                ])
            }).collect(),
        value: "4294967295",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "u32-min" => Constant {
        name: "The smallest value of 32-bit unsigned integer",
        tokens:
            || iproduct!(["u32", "uint32", "uint32_t"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:u32-min",
                    key; "keyword:u32-min"
                ])
            }).collect(),
        value: "0",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "u64-max" => Constant {
        name: "The largest value of 64-bit unsigned integer",
        tokens:
            || iproduct!(["u64", "uint64", "uint64_t"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:u64-max",
                    key; "keyword:u64-max"
                ])
            }).collect(),
        value: "18446744073709551615",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "u64-min" => Constant {
        name: "The smallest value of 64-bit unsigned integer",
        tokens:
            || iproduct!(["u64", "uint64", "uint64_t"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:u64-min",
                    key; "keyword:u64-min"
                ])
            }).collect(),
        value: "0",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "u128-max" => Constant {
        name: "The largest value of 128-bit unsigned integer",
        tokens:
            || iproduct!(["u128", "uint128", "uint128_t"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:u128-max",
                    key; "keyword:u128-max"
                ])
            }).collect(),
        value: "340282366920938463463374607431768211455",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "u128-min" => Constant {
        name: "The smallest value of 128-bit unsigned integer",
        tokens:
            || iproduct!(["u128", "uint128", "uint128_t"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:u128-min",
                    key; "keyword:u128-min"
                ])
            }).collect(),
        value: "0",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "i8-max" => Constant {
        name: "The largest value of 16-bit signed integer",
        tokens:
            || iproduct!(["i8", "int8", "int8_t"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:i8-max",
                    key; "keyword:i8-max"
                ])
            }).collect(),
        value: "127",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "i8-min" => Constant {
        name: "The smallest value of 16-bit signed integer",
        tokens:
            || iproduct!(["i8", "int8", "int8_t"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:i8-min",
                    key; "keyword:i8-min"
                ])
            }).collect(),
        value: "-128",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "i16-max" => Constant {
        name: "The largest value of 16-bit signed integer",
        tokens:
            || iproduct!(["i16", "int16", "int16_t"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:i16-max",
                    key; "keyword:i16-max"
                ])
            }).collect(),
        value: "32767",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "i16-min" => Constant {
        name: "The smallest value of 16-bit signed integer",
        tokens:
            || iproduct!(["i16", "int16", "int16_t"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:i16-min",
                    key; "keyword:i16-min"
                ])
            }).collect(),
        value: "-32768",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "i32-max" => Constant {
        name: "The largest value of 32-bit signed integer",
        tokens:
            || iproduct!(["i32", "int32", "int32_t"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:i32-max",
                    key; "keyword:i32-max"
                ])
            }).collect(),
        value: "2147483647",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "i32-min" => Constant {
        name: "The smallest value of 32-bit signed integer",
        tokens:
            || iproduct!(["i32", "int32", "int32_t"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:i32-min",
                    key; "keyword:i32-min"
                ])
            }).collect(),
        value: "-2147483648",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "i64-max" => Constant {
        name: "The largest value of 64-bit signed integer",
        tokens:
            || iproduct!(["i64", "int64", "int64_t"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:i64-max",
                    key; "keyword:i64-max"
                ])
            }).collect(),
        value: "9223372036854775807",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "i64-min" => Constant {
        name: "The smallest value of 64-bit signed integer",
        tokens:
            || iproduct!(["i64", "int64", "int64_t"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:i64-min",
                    key; "keyword:i64-min"
                ])
            }).collect(),
        value: "-9223372036854775808",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "i128-max" => Constant {
        name: "The largest value of 128-bit signed integer",
        tokens:
            || iproduct!(["i128", "int128", "int128_t"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:i128-max",
                    key; "keyword:i128-max"
                ])
            }).collect(),
        value: "170141183460469231731687303715884105727",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "i128-min" => Constant {
        name: "The smallest value of 128-bit signed integer",
        tokens:
            || iproduct!(["i128", "int128", "int128_t"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:i128-min",
                    key; "keyword:i128-min"
                ])
            }).collect(),
        value: "-170141183460469231731687303715884105728",
        scale: 0,
        unit: None,
        is_exact: true,
    },
    "f32-max" => Constant {
        name: "The largest value of 32-bit floating point number",
        tokens:
            || iproduct!(["f32", "float32", "float"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:f32-max",
                    key; "keyword:f32-max"
                ])
            }).collect(),
        value: "3.40282347",
        scale: 38,
        unit: None,
        is_exact: true,
    },
    "f32-min" => Constant {
        name: "The smallest value of 32-bit floating point number",
        tokens:
            || iproduct!(["f32", "float32", "float"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:f32-min",
                    key; "keyword:f32-min"
                ])
            }).collect(),
        value: "-3.40282347",
        scale: 38,
        unit: None,
        is_exact: true,
    },
    "f64-max" => Constant {
        name: "The largest value of 64-bit floating point number",
        tokens:
            || iproduct!(["f64", "float64", "double"], ["max", "maximum", "largest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:f64-max",
                    key; "keyword:f64-max"
                ])
            }).collect(),
        value: "1.7976931348623157",
        scale: 308,
        unit: None,
        is_exact: true,
    },
    "f64-min" => Constant {
        name: "The smallest value of 64-bit floating point number",
        tokens:
            || iproduct!(["f64", "float64", "double"], ["min", "minimum", "smallest"]).map(|(ty, key)| {
                tk!([
                    ty; "keyword:f64-min",
                    key; "keyword:f64-min"
                ])
            }).collect(),
        value: "-1.7976931348623157",
        scale: 308,
        unit: None,
        is_exact: true,
    },
    "pi" => Constant {
        name: "π",
        tokens: || vec![
            tk!([
                "pi"; "keyword:pi"
            ]),
            tk!([
                "π"; "keyword:pi"
            ]),
        ],
        value: "3.14159265358979323846264338327950288419716939937510",
        scale: 0,
        unit: None,
        is_exact: false,
    },
    "speed-of-light" => Constant {
        name: "The speed of light in vacuum",
        tokens: || vec![
            tk!([
                "The",
                "speed"; "keyword:speed-of-light",
                "of",
                "light"; "keyword:speed-of-light"
            ]),
            tk!([
                "How",
                "fast",
                "the",
                "speed"; "keyword:speed-of-light",
                "of",
                "light"; "keyword:speed-of-light"
            ]),
            tk!([
                "The",
                "speed"; "keyword:speed-of-light",
                "of",
                "light"; "keyword:speed-of-light",
                "in",
                "vacuum"
            ]),
        ],
        value: "299792458",
        scale: 0,
        unit: Some("m/s"),
        is_exact: true,
    },
    "electron-mass" => Constant {
        name: "Electron mass",
        tokens: || vec![
            tk!([
                "electron"; "keyword:electron-mass",
                "mass"; "keyword:electron-mass"
            ]),
            tk!([
                "mass"; "keyword:electron-mass",
                "of",
                "electron"; "keyword:electron-mass"
            ]),
            tk!([
                "a",
                "mass"; "keyword:electron-mass",
                "of",
                "an",
                "electron"; "keyword:electron-mass"
            ])
        ],
        value: "9.109383701528",
        scale: -31,
        unit: Some("kg"),
        is_exact: true,
    },
    "proton-mass" => Constant {
        name: "Proton mass",
        tokens:|| vec![
            tk!([
                "proton"; "keyword:proton-mass",
                "mass"; "keyword:proton-mass"
            ]),
            tk!([
                "mass"; "keyword:proton-mass",
                "of",
                "proton"; "keyword:proton-mass"
            ]),
            tk!([
                "a",
                "mass"; "keyword:proton-mass",
                "of",
                "a",
                "proton"; "keyword:proton-mass"
            ])
        ],
        value: "1.6726219236951",
        scale: -27,
        unit: Some("kg"),
        is_exact: true,
    },
    "neutron-mass" => Constant {
        name: "Neutron mass",
        tokens:|| vec![
            tk!([
                "neutron"; "keyword:neutron-mass",
                "mass"; "keyword:neutron-mass"
            ]),
            tk!([
                "mass"; "keyword:neutron-mass",
                "of",
                "neutron"; "keyword:neutron-mass"
            ]),
            tk!([
                "a",
                "mass"; "keyword:neutron-mass",
                "of",
                "a",
                "neutron"; "keyword:neutron-mass"
            ])
        ],
        value: "1.6749274980495",
        scale: -27,
        unit: Some("kg"),
        is_exact: true,
    },
    "muon-mass" => Constant {
        name: "Muon mass",
        tokens:|| vec![
            tk!([
                "muon"; "keyword:muon-mass",
                "mass"; "keyword:muon-mass"
            ]),
            tk!([
                "mass"; "keyword:muon-mass",
                "of",
                "muon"; "keyword:muon-mass"
            ]),
            tk!([
                "a",
                "mass"; "keyword:muon-mass",
                "of",
                "a",
                "muon"; "keyword:muon-mass"
            ])
        ],
        value: "1.88353162742",
        scale: -28,
        unit: Some("kg"),
        is_exact: true,
    },
    "tau-mass" => Constant {
        name: "Tau mass",
        tokens:|| vec![
            tk!([
                "tau"; "keyword:tau-mass",
                "mass"; "keyword:tau-mass"
            ]),
            tk!([
                "mass"; "keyword:tau-mass",
                "of",
                "tau"; "keyword:tau-mass"
            ]),
            tk!([
                "a",
                "mass"; "keyword:tau-mass",
                "of",
                "a",
                "tau"; "keyword:tau-mass"
            ])
        ],
        value: "3.1675421",
        scale: -27,
        unit: Some("kg"),
        is_exact: true,
    },
    "top-quark-mass" => Constant {
        name: "Top quark mass",
        tokens:|| vec![
            tk!([
                "top"; "keyword:top-quark-mass",
                "quark"; "keyword:top-quark-mass",
                "mass"; "keyword:top-quark-mass"
            ]),
            tk!([
                "mass"; "keyword:top-quark-mass",
                "of",
                "top"; "keyword:top-quark-mass",
                "quark"; "keyword:top-quark-mass"
            ]),
            tk!([
                "a",
                "mass"; "keyword:top-quark-mass",
                "of",
                "a",
                "top"; "keyword:top-quark-mass",
                "quark"; "keyword:top-quark-mass"
            ])
        ],
        value: "3.078453",
        scale: -25,
        unit: Some("kg"),
        is_exact: true,
    }
};
