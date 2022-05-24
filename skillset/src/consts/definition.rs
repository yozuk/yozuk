use super::Constant;
use maplit::hashmap;
use std::collections::HashMap;
use yozuk_sdk::prelude::*;

lazy_static::lazy_static! {
    pub static ref DEFINITIONS: HashMap<&'static str, Constant> = {
        hashmap![
            "pi" => Constant {
                name: "π",
                tokens: vec![
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
                tokens: vec![
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
                tokens: vec![
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
        ]
    };
}
