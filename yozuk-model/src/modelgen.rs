#![cfg(feature = "modelgen")]

use super::labeler::*;
use anyhow::{bail, Result};
use bytes::Bytes;
use crfsuite::{Algorithm, Attribute, GraphicalModel, Trainer};
use itertools::multiunzip;
use nanoid::nanoid;
use rayon::prelude::*;
use std::{collections::VecDeque, env, fs::File, io::Read, iter};
use yozuk_sdk::model::*;
use yozuk_sdk::prelude::*;

pub fn modelgen(skills: &[NamedSkillEntry], env: &Environment) -> Result<ModelSet> {
    let mut keys = skills
        .iter()
        .map(|item| item.key.to_string())
        .collect::<Vec<_>>();
    keys.sort();

    let labelers = skills
        .par_iter()
        .flat_map(|item| {
            (item.entry.init)(env, &Default::default())
                .unwrap()
                .labelers
        })
        .collect::<Vec<_>>();

    let labeler = FeatureLabeler::new(&labelers);

    let dataset = skills
        .par_iter()
        .map(|item| TrainingData {
            key: item.key.to_string(),
            skills: vec![(item.entry.init)(env, &Default::default()).unwrap()],
            negative_skills: skills
                .par_iter()
                .filter(|neg| neg.key != item.key)
                .map(|neg| (neg.entry.init)(env, &Default::default()).unwrap())
                .collect(),
        })
        .filter_map(|item| learn(item, &labeler).ok())
        .collect::<Vec<_>>();

    let mut ranges = vec![0..0; keys.len()];
    let mut data = Vec::<u8>::new();

    for (key, mut item) in dataset {
        let index = keys.binary_search(&key).unwrap();
        ranges[index] = data.len()..data.len() + item.len();
        data.append(&mut item);
    }

    Ok(ModelSet::new(
        data,
        keys.into_iter().zip(ranges.into_iter()),
    ))
}

fn learn(item: TrainingData, labeler: &FeatureLabeler) -> Result<(String, Vec<u8>)> {
    let mut tr = Trainer::new(false);
    tr.select(Algorithm::LBFGS, GraphicalModel::CRF1D).unwrap();

    let seq = item
        .skills
        .iter()
        .map(|skill| (&skill.corpora, &skill.preprocessors))
        .flat_map(|(corpora, preps)| {
            corpora.iter().flat_map(move |corpus| {
                let weight = corpus.weight();
                corpus.training_data().into_iter().map(move |tokens| {
                    preps
                        .iter()
                        .fold(tokens, |tokens, prep| prep.preprocess(tokens))
                        .into_iter()
                        .map(|token| WeightedToken::new(token, weight))
                        .collect::<Vec<_>>()
                })
            })
        })
        .flat_map(generate_wordiness)
        .map(|data| {
            let (yseq, words, weights): (Vec<_>, Vec<_>, Vec<_>) =
                multiunzip(data.into_iter().map(|token| {
                    (
                        token.tag.clone(),
                        Token {
                            data: token.data,
                            media_type: token.media_type,
                            tag: token.tag,
                        },
                        token.weight,
                    )
                }));

            let xseq = labeler
                .label_features(&words)
                .into_iter()
                .zip(weights)
                .map(|(features, weight)| {
                    features
                        .into_iter()
                        .map(|feature| Attribute::new(feature.to_string(), weight))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            (xseq, yseq)
        })
        .collect::<Vec<_>>();

    if seq.is_empty() {
        bail!("no training data");
    }

    for (xseq, yseq) in &seq {
        tr.append(xseq, yseq, 0)?;
    }

    let seq = item
        .negative_skills
        .iter()
        .map(|skill| (&skill.corpora, &skill.preprocessors))
        .flat_map(|(corpora, preps)| {
            corpora.iter().flat_map(move |corpus| {
                let weight = corpus.weight();
                corpus.training_data().into_iter().map(move |tokens| {
                    preps
                        .iter()
                        .fold(tokens, |tokens, prep| prep.preprocess(tokens))
                        .into_iter()
                        .map(|token| WeightedToken::new(token, weight))
                        .collect::<Vec<_>>()
                })
            })
        })
        .flat_map(generate_wordiness)
        .map(|data| {
            let (yseq, words, weights): (Vec<_>, Vec<_>, Vec<_>) =
                multiunzip(data.into_iter().map(|token| {
                    (
                        if token.tag == "-" {
                            "-".to_string()
                        } else {
                            "*".to_string()
                        },
                        Token {
                            data: token.data,
                            media_type: token.media_type,
                            tag: token.tag,
                        },
                        token.weight,
                    )
                }));

            let xseq = labeler
                .label_features(&words)
                .into_iter()
                .zip(weights)
                .map(|(features, weight)| {
                    features
                        .into_iter()
                        .map(|feature| Attribute::new(feature.to_string(), 0.01 * weight))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            (xseq, yseq)
        });

    for (xseq, yseq) in seq {
        tr.append(&xseq, &yseq, 0)?;
    }

    let filename = format!(
        "{}/{}.crfsuite",
        env::temp_dir().as_os_str().to_str().unwrap(),
        nanoid!()
    );

    tr.train(&filename, -1)?;
    let mut file = File::open(&filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok((item.key.to_string(), data))
}

struct TrainingData {
    key: String,
    skills: Vec<Skill>,
    negative_skills: Vec<Skill>,
}

fn generate_wordiness(data: Vec<WeightedToken>) -> impl Iterator<Item = Vec<WeightedToken>> {
    generate_wordiness_greetings(&data).chain(iter::once(data))
}

fn generate_wordiness_greetings(
    tokens: &[WeightedToken],
) -> impl Iterator<Item = Vec<WeightedToken>> {
    let original = tokens.iter().cloned().collect::<VecDeque<_>>();
    let weight: f64 = tokens.iter().map(|token| token.weight).sum::<f64>() / tokens.len() as f64;

    let mut greetings = Vec::new();
    let mut data = original.clone();
    data.push_front(WeightedToken::new(tk!("Yozuk,"), weight));
    greetings.push(data.into_iter().collect::<Vec<_>>());

    let mut data = original;
    data.push_front(WeightedToken::new(tk!("Yozuk,"), weight));
    data.push_front(WeightedToken::new(tk!("Hi"), weight));
    greetings.push(data.into_iter().collect::<Vec<_>>());

    greetings.into_iter()
}

#[derive(Debug, Clone)]
pub struct WeightedToken {
    pub data: Bytes,
    pub media_type: MediaTypeBuf,
    pub tag: String,
    pub weight: f64,
}

impl WeightedToken {
    pub fn new(token: Token, weight: f64) -> Self {
        Self {
            weight,
            ..Self::from(token)
        }
    }
}

impl Default for WeightedToken {
    fn default() -> Self {
        Self {
            data: Bytes::new(),
            media_type: media_type!(TEXT / PLAIN).into(),
            tag: String::new(),
            weight: 1.0,
        }
    }
}

impl From<Token> for WeightedToken {
    fn from(token: Token) -> Self {
        Self {
            data: token.data,
            media_type: token.media_type,
            tag: token.tag,
            ..Default::default()
        }
    }
}
