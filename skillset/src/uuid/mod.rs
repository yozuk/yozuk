use clap::Parser;
use itertools::iproduct;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::iter;
use uuid::Uuid;
use yozuk_helper_english::{normalized_eq, pluralize, NumeralTokenParser};
use yozuk_helper_preprocessor::TokenMerger;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"uJrzxcw12a52vwpLOgnCd",
    init: |_| {
        Skill::builder()
            .add_preprocessor(TokenMerger::new(NumeralTokenParser))
            .add_labeler(UuidLabeler)
            .add_corpus(UuidCorpus)
            .add_suggestions(UuidSuggestions)
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
        iproduct!(
            ["generate", "new"],
            ["uuid", "guid"],
            ["lower", "upper", "lowercase", "uppercase"]
        )
        .flat_map(|(verb, name, case)| {
            vec![
                tk!([
                    verb,
                    name; "command",
                    case; "input:charset"
                ]),
                tk!([
                    "please",
                    verb,
                    name; "command",
                    case; "input:charset"
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
                            name; "command"
                        ]),
                        tk!([
                            "please",
                            verb,
                            format!("{}", count); "input:count",
                            name; "command"
                        ]),
                    ]
                },
            ),
        )
        .chain(
            iproduct!(
                ["generate", "new"],
                ["uuid", "guid"],
                ["lower", "upper", "lowercase", "uppercase"],
                1..=10
            )
            .flat_map(|(verb, name, case, count)| {
                vec![
                    tk!([
                        verb,
                        format!("{}", count); "input:count",
                        case; "input:charset",
                        name; "command"
                    ]),
                    tk!([
                        "please",
                        verb,
                        format!("{}", count); "input:count",
                        case; "input:charset",
                        name; "command"
                    ]),
                ]
            }),
        )
        .chain(["uuid", "guid"].map(|name| tk!([name; "command"])))
        .chain(vec![tk!([
            "generate",
            format!("{}", Uuid::nil()); "input:uuid"
        ])])
        .collect()
    }
}

#[derive(Debug)]
pub struct UuidSuggestions;

impl Suggestions for UuidSuggestions {
    fn suggestions(&self, seed: u64, args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        let count = args
            .iter()
            .find(|arg| arg.tag == "input:count")
            .and_then(|arg| arg.as_str().parse::<u8>().ok())
            .filter(|&n| n > 0);
        let mut rng = StdRng::seed_from_u64(seed);
        let n = count.unwrap_or_else(|| rng.gen_range(2..=10));
        vec![format!("Generate {} UUIDs", n)]
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
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        if !args
            .iter()
            .any(|arg| arg.tag == "command" && normalized_eq(arg.as_str(), &["UUID", "GUID"], 0))
        {
            return None;
        }
        let upper = if args.iter().any(|arg| {
            arg.tag == "input:charset" && normalized_eq(arg.as_str(), &["upper", "uppercase"], 1)
        }) {
            Some("--upper")
        } else {
            None
        };
        let count = args
            .iter()
            .find(|arg| arg.tag == "input:count")
            .and_then(|arg| arg.as_str().parse::<usize>().ok())
            .unwrap_or(1);
        Some(
            CommandArgs::new()
                .add_args(["-n".to_string(), count.to_string()])
                .add_args_iter(upper),
        )
    }
}

#[cfg(feature = "wild")]
const MAX_COUNT: usize = u16::MAX as _;

#[cfg(not(feature = "wild"))]
const MAX_COUNT: usize = 32;

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
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/uuid/")?;
        if args.n > MAX_COUNT {
            return Err(Output::new()
                .set_title("UUID Generator")
                .add_block(block::Comment::new().set_text(format!(
                    "Too large number of the requested UUIDs (Limit: {}).",
                    MAX_COUNT
                )))
                .add_metadata(docs)
                .into());
        }
        let list = iter::repeat_with(|| format!("{}", Uuid::new_v4()))
            .take(args.n)
            .map(|id| {
                if args.upper {
                    id.to_ascii_uppercase()
                } else {
                    id
                }
            })
            .collect::<Vec<_>>();
        Ok(Output::new()
            .set_title("UUID Generator")
            .add_blocks_iter(vec![
                Block::Comment(block::Comment::new().set_text(format!(
                    "Generating {} {}",
                    args.n,
                    pluralize("UUID", args.n)
                ))),
                Block::Data(block::Data::new().set_text_data(list.join("\n"))),
            ])
            .add_metadata(docs))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, default_value_t = 1)]
    pub n: usize,

    #[clap(long)]
    pub upper: bool,
}
