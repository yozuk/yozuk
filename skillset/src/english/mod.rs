#![forbid(unsafe_code)]
#![deny(clippy::all)]

use rayon::prelude::*;
use yozuk_helper_english::singularize;
use yozuk_sdk::prelude::*;

mod stopwords;
mod thesaurus;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"TT6SRME5RFLd13Rl6gwL4",
    config_schema: None,
    init: |_, _| Skill::builder().add_labeler(EnglishLabeler).build(),
};

fn label_stop_words(token: &Token) -> impl Iterator<Item = Feature> {
    if stopwords::STOPWORDS
        .iter()
        .any(|&word| word == token.as_utf8().to_lowercase().as_str())
    {
        vec![Feature {
            name: "english:stop".into(),
            non_entity: true,
            ..Default::default()
        }]
    } else {
        vec![]
    }
    .into_iter()
}

fn label_synonyms(token: &Token) -> impl Iterator<Item = Feature> {
    let text = singularize(&token.as_utf8().to_lowercase());
    thesaurus::SYNONYMS
        .par_iter()
        .find_any(|list| list.contains(&text.as_str()))
        .map(|list| Feature {
            name: format!("english:synonym:{}", list.get(0).unwrap()),
            ..Default::default()
        })
        .into_iter()
}

#[derive(Debug)]
pub struct EnglishLabeler;

impl Labeler for EnglishLabeler {
    fn label_features(&self, input: &[Token]) -> Vec<Vec<Feature>> {
        input
            .iter()
            .map(|token| {
                label_stop_words(token)
                    .chain(label_synonyms(token))
                    .collect()
            })
            .collect()
    }
}
