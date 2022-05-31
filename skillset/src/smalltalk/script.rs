use super::Script;
use maplit::hashmap;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use yozuk_sdk::prelude::*;

pub fn scripts() -> &'static HashMap<&'static str, Script> {
    static INSTANCE: OnceCell<HashMap<&'static str, Script>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        hashmap![
            "42" => Script {
                title: Some("Deep Thought"),
                tokens: vec![
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
                 again seven and a half million years later."][..],
            },
            "early-bird" => Script {
                title: None,
                tokens: vec![
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
                responses: &["As you see, I'm a night owl."][..],
            }
        ]
    })
}
