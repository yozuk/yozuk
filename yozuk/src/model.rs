use super::FeatureLabeler;
use bytes::Bytes;
use yozuk_model::*;
use yozuk_sdk::prelude::*;

pub struct ModelEntry {
    tagger: Tagger,
}

impl ModelEntry {
    pub fn new(data: Bytes) -> Self {
        Self {
            tagger: Tagger::new(data),
        }
    }

    pub fn tag<I, T>(&self, features: I) -> Vec<String>
    where
        I: AsRef<[T]>,
        T: AsRef<[Feature]>,
    {
        self.tagger.tag(features)
    }

    pub fn tag_tokens(&self, labeler: &FeatureLabeler, tokens: &[Token]) -> Vec<Token> {
        let features = labeler.label_features(tokens);
        let tags = self.tag(&features);

        tokens
            .iter()
            .zip(tags.into_iter())
            .map(|(token, tag)| Token {
                tag: if token.tag.is_empty() {
                    tag
                } else {
                    token.tag.clone()
                },
                ..token.clone()
            })
            .collect()
    }
}
