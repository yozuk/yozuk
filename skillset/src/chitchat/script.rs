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
    responses: |_| vec!["Computing the answer to your question will take a little while. Please ask me \
        again seven and a half million years later.".into()],
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
    responses: |_| vec!["As you see, I'm a night owl.".into()],
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
    responses: |_| vec!["See https://docs.yozuk.com/ for documentation.".into()],
},
"thanks" => Script {
    title: None,
    tokens: || vec![
        tk!([
            "Thanks"; "keyword:thanks"
        ]),
        tk!([
            "Thank"; "keyword:thanks",
            "you"
        ]),
    ],
    responses: |_| vec![
        "Glad to help!".into(),
        "I'm happy to help.".into(),
        "No worries. Don't forget to leave a star on https://github.com/yozuk/yozuk !".into()
    ],
},
"hello" => Script {
    title: None,
    tokens: || vec![
        tk!([
            "Hello"; "keyword:hello"
        ]),
        tk!([
            "Hi"; "keyword:hello"
        ]),
    ],
    responses: |user| vec![
        "Hi there!".into(),
        "Hello! How may I help you?".into()
    ].into_iter().chain(
        user.username.as_ref().map(|name| format!("Hi {name}!"))
    ).collect(),
}
};
