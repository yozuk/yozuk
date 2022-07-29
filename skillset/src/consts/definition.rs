use super::Constant;
use phf::phf_map;
use yozuk_sdk::prelude::*;

pub static DEFINITIONS: phf::Map<&'static str, Constant> = phf_map! {
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
