use super::Script;
use phf::phf_map;
use yozuk_sdk::prelude::*;

pub static SCRIPTS: phf::Map<&'static str, Script> = phf_map! {
    "42" => Script {
    title: Some("Deep Thought"),
    tokens: || vec![
        tk!([
            "life"; "keyword:42",
            "universe"; "keyword:42",
            "everything"; "keyword:42"
        ]),
        tk!([
            "Life"; "keyword:42",
            "universe"; "keyword:42",
            "everything"; "keyword:42"
        ]),
        tk!([
            "Life,"; "keyword:42",
            "the",
            "universe"; "keyword:42",
            "and",
            "everything"; "keyword:42"
        ]),
        tk!([
            "The", "answer", "to",
            "Life,"; "keyword:42",
            "universe"; "keyword:42",
            "and",
            "everything"; "keyword:42"
        ]),
        tk!([
            "The", "answer", "to",
            "Life,"; "keyword:42",
            "universe"; "keyword:42",
            "and",
            "everything"; "keyword:42"
        ]),
        tk!([
            "The", "answer", "to",
            "Life,"; "keyword:42",
            "the",
            "universe"; "keyword:42",
            "and",
            "everything"; "keyword:42"
        ]),
    ],
    responses: &["Computing the answer to your question will take a little while. Please ask me \
        again seven and a half million years later."],
},
"early-bird" => Script {
    title: None,
    tokens: || vec![
        tk!([
            "Do",
            "you",
            "wake"; "keyword:early-bird",
            "up"; "keyword:early-bird",
            "early"; "keyword:early-bird"
        ]),
        tk!([
            "Are",
            "you",
            "an",
            "early"; "keyword:early-bird",
            "bird"; "keyword:early-bird"
        ]),
    ],
    responses: &["As you see, I'm a night owl."],
},
"help" => Script {
    title: Some("Help"),
    tokens: || vec![
        tk!([
            "help"; "keyword:help"
        ]),
        tk!([
            "documentation"; "keyword:help"
        ]),
        tk!([
            "docs"; "keyword:help"
        ]),
    ],
    responses: &["See https://docs.yozuk.com/ for documentation."],
}
};
