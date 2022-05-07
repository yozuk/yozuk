#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use itertools::iproduct;
use std::iter;
use uuid::Uuid;
use yozuk_helper_english::{normalized_eq, pluralize};
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"tzUyypo_Lz2T95dun91YX",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_labeler(UuidLabeler)
            .add_corpus(UuidCorpus)
            .add_suggests(UuidSuggests)
            .add_translator(UuidTranslator)
            .set_command(UuidCommand)
            .build()
    },
};

fn label_uuid(token: &Token) -> impl Iterator<Item = Feature> {
    Uuid::parse_str(token.as_str())
        .ok()
        .map(|_| Feature {
            name: "format:uuid".into(),
            non_entity: true,
            ..Default::default()
        })
        .into_iter()
}

#[derive(Debug)]
pub struct UuidCorpus;

impl Corpus for UuidCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        iproduct!(["generate", "new"], ["uuid", "guid"])
            .flat_map(|(verb, name)| {
                vec![
                    tk!([
                        verb,
                        name; "command:uuid"
                    ]),
                    tk!([
                        "please",
                        verb,
                        name; "command:uuid"
                    ]),
                ]
            })
            .chain(
                iproduct!(["generate", "new"], ["uuid", "guid"], 1..=10).flat_map(
                    |(verb, name, count)| {
                        vec![
                            tk!([
                                verb,
                                format!("{}", count); "input:count",
                                name; "command:uuid"
                            ]),
                            tk!([
                                "please",
                                verb,
                                format!("{}", count); "input:count",
                                name; "command:uuid"
                            ]),
                        ]
                    },
                ),
            )
            .chain(["uuid", "guid"].map(|name| tk!([name; "command:uuid"])))
            .chain(vec![tk!([
                "generate",
                format!("{}", Uuid::nil()); "input:uuid"
            ])])
            .collect()
    }
}

#[derive(Debug)]
pub struct UuidSuggests;

impl Suggests for UuidSuggests {
    fn suggests(&self, _input: &[Token]) -> Vec<String> {
        [
            "UUID",
            "GUID",
            "Genarate UUID",
            "Generate GUID",
            "New UUID",
            "New GUID",
        ]
        .into_iter()
        .map(Into::into)
        .collect()
    }
}

#[derive(Debug)]
pub struct UuidLabeler;

impl Labeler for UuidLabeler {
    fn label_features(&self, input: &[Token]) -> Vec<Vec<Feature>> {
        input
            .iter()
            .map(|token| label_uuid(token).collect())
            .collect()
    }
}

#[derive(Debug)]
pub struct UuidTranslator;

impl Translator for UuidTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        if !args.iter().any(|arg| {
            arg.tag == "command:uuid" && normalized_eq(arg.as_str(), &["UUID", "GUID"], 0)
        }) {
            return None;
        }
        let count = args
            .iter()
            .find(|arg| arg.tag == "input:count")
            .and_then(|arg| arg.as_str().parse::<usize>().ok())
            .unwrap_or(1);
        Some(CommandArgs::new().add_args(["-n".to_string(), count.to_string()]))
    }
}

const MAX_COUNT: usize = 30;

#[derive(Debug)]
pub struct UuidCommand;

impl Command for UuidCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        if args.n > MAX_COUNT {
            return Err(Output::new()
                .set_title("UUID Generator")
                .add_block(Block::Comment(block::Comment::new().set_text(format!(
                    "Too large number of the requested UUIDs (Limit: {}).",
                    MAX_COUNT
                ))))
                .into());
        }
        let list = iter::repeat_with(|| format!("{}", Uuid::new_v4()))
            .take(args.n)
            .collect::<Vec<_>>();
        Ok(Output::new().set_title("UUID Generator").add_blocks(vec![
            Block::Comment(block::Comment::new().set_text(format!(
                "Generating {} {}",
                args.n,
                pluralize("UUID", args.n)
            ))),
            Block::Data(block::Data::new().set_text_data(list.join("\n"))),
        ]))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, default_value_t = 1)]
    pub n: usize,
}
