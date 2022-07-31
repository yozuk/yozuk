use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"0GIQkXhytKtGhqwFg7Wzj",
    init: |_| Skill::builder().add_labeler(CoreLabeler).build(),
};

pub struct CoreLabeler;

impl Labeler for CoreLabeler {
    fn label_features(&self, input: &[Token]) -> Vec<Vec<Feature>> {
        input
            .iter()
            .map(|_| {
                vec![
                    Feature {
                        name: "media:subtype:plain".to_string(),
                        ..Default::default()
                    },
                    Feature {
                        name: "media:type:text".to_string(),
                        ..Default::default()
                    },
                ]
            })
            .collect()
    }
}
