use super::Constant;
use yozuk_sdk::prelude::*;
use std::collections::HashMap;
use maplit::hashmap;

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
                        "The",
                        "speed"; "keyword:speed-of-light",
                        "of",
                        "light"; "keyword:speed-of-light",
                        "in",
                        "vaccum"
                    ]),
                ],
                value: "299792458",
                unit: Some("m/s"),
                is_exact: true,
            }
        ]
    };
}
