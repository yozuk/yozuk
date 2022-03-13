#![forbid(unsafe_code)]

use bigdecimal::BigDecimal;
use bigdecimal::Signed;
use std::str::FromStr;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"nSz49UpiDtQLWUfAUZEq1",
    config_schema: None,
    init: |_, _| Skill::builder().add_labeler(NumericLabeler).build(),
};

fn label_numeric(token: &Token) -> impl Iterator<Item = Feature> {
    let mut features = Vec::new();
    if let Ok(n) = BigDecimal::from_str(&token.as_utf8()) {
        features.push(Feature {
            name: "numeric".into(),
            ..Default::default()
        });
        features.push(if n.is_positive() {
            Feature {
                name: "numeric:positive".into(),
                ..Default::default()
            }
        } else if n.is_negative() {
            Feature {
                name: "numeric:negative".into(),
                ..Default::default()
            }
        } else {
            Feature {
                name: "numeric:zero".into(),
                ..Default::default()
            }
        });
        features.push(if n.is_integer() {
            Feature {
                name: "numeric:integer".into(),
                ..Default::default()
            }
        } else {
            Feature {
                name: "numeric:float".into(),
                ..Default::default()
            }
        });
    }
    features.into_iter()
}

#[derive(Debug)]
pub struct NumericLabeler;

impl Labeler for NumericLabeler {
    fn label_features(&self, input: &[Token]) -> Vec<Vec<Feature>> {
        input
            .iter()
            .map(|token| label_numeric(token).collect())
            .collect()
    }
}
