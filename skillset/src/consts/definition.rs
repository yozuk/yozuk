use super::Constant;
use yozuk_sdk::prelude::*;

lazy_static::lazy_static! {
    pub static ref DEFINITIONS: Vec<Constant> = {
        vec![
            Constant {
                name: "The speed of light in vacuum",
                tokens: vec![
                    tk!([
                        "The",
                        "speed"; "keyword",
                        "of",
                        "light"; "keyword"
                    ]),
                    tk!([
                        "The",
                        "speed"; "keyword",
                        "of",
                        "light"; "keyword",
                        "in",
                        "vaccum"
                    ]),
                ],
                value: "299792458",
                unit: Some("m/s"),
            }
        ]
    };
}
