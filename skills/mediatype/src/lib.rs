#![forbid(unsafe_code)]

use mediatype::ReadParams;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"zj6AkibVnIy0xBLEosYdH",
    config_schema: None,
    init: |_, _| Skill::builder().add_labeler(CoreLabeler).build(),
};

fn label_media_type(token: &Token) -> impl Iterator<Item = Feature> {
    let mut features = Vec::new();
    let media_type = &token.media_type;
    features.push(Feature {
        name: format!("media:type:{}", media_type.ty()),
        ..Default::default()
    });
    features.push(Feature {
        name: format!("media:subtype:{}", media_type.subty()),
        ..Default::default()
    });
    if let Some(suffix) = media_type.suffix() {
        features.push(Feature {
            name: format!("media:suffix:{}", suffix),
            ..Default::default()
        });
    }
    let mut params = media_type
        .params()
        .map(|(key, value)| Feature {
            name: format!("media:params:{}={}", key, value),
            ..Default::default()
        })
        .collect();
    features.append(&mut params);
    features.into_iter()
}

#[derive(Debug)]
pub struct CoreLabeler;

impl Labeler for CoreLabeler {
    fn label_features(&self, input: &[Token]) -> Vec<Vec<Feature>> {
        input
            .iter()
            .map(|token| label_media_type(token).collect())
            .collect()
    }
}
