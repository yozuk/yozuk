use yozuk_helper_english::normalize;
use yozuk_sdk::prelude::*;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

const MAXIMUM_SHANNON_ENTROPY: f32 = 3.0;
const MAXIMUM_TOKEN_LENGTH: usize = 20;

pub struct FeatureLabeler<'a> {
    labelers: &'a [Box<dyn Labeler>],
}

impl<'a> FeatureLabeler<'a> {
    pub fn new(labelers: &'a [Box<dyn Labeler>]) -> Self {
        Self { labelers }
    }

    pub fn label_features(&self, input: &[Token]) -> Vec<Vec<Feature>> {
        #[cfg(feature = "rayon")]
        let iter = self.labelers.par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = self.labelers.iter();

        let iter = iter.map(|labeler| labeler.label_features(input));

        #[cfg(feature = "rayon")]
        let skill_features = iter.reduce(Vec::new, merge_featurs);
        #[cfg(not(feature = "rayon"))]
        let skill_features = iter.reduce(merge_featurs).unwrap_or_default();

        let features = input
            .iter()
            .filter(|token| token.shannon_entropy() <= MAXIMUM_SHANNON_ENTROPY)
            .filter_map(|token| punycode::encode(&normalize(token.as_utf8())).ok())
            .map(|text| {
                if text.len() <= MAXIMUM_TOKEN_LENGTH {
                    vec![Feature {
                        name: format!("token:{}", text),
                        ..Default::default()
                    }]
                } else {
                    vec![]
                }
            })
            .collect::<Vec<_>>();

        let features = merge_featurs(skill_features, features);

        let mut nighbores: Vec<Vec<Feature>> = vec![vec![]; features.len()];

        for (i, list) in features.iter().enumerate() {
            for offset in [-2, -1, 1, 2] {
                let index = i as isize + offset;
                if index >= 0 && index < nighbores.len() as isize {
                    let index = index as usize;
                    if !features[index].iter().any(|f| f.non_entity) {
                        nighbores[index as usize].append(
                            &mut list
                                .iter()
                                .map(|f| Feature {
                                    pos: -offset,
                                    ..f.clone()
                                })
                                .collect(),
                        );
                    }
                }
            }
        }

        features
            .into_iter()
            .zip(nighbores.into_iter())
            .map(|(a, b)| a.into_iter().chain(b.into_iter()).collect())
            .collect()
    }
}

fn merge_featurs(mut a: Vec<Vec<Feature>>, mut b: Vec<Vec<Feature>>) -> Vec<Vec<Feature>> {
    let len = a.len().max(b.len());
    a.resize_with(len, Vec::new);
    b.resize_with(len, Vec::new);
    a.into_iter()
        .zip(b.into_iter())
        .map(|(mut a, mut b)| {
            a.append(&mut b);
            a
        })
        .collect()
}
