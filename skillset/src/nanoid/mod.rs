use clap::Parser;
use itertools::iproduct;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::iter;
use yozuk_helper_english::{normalized_eq, NumeralTokenParser};
use yozuk_sdk::prelude::*;
use yozuk_sdk::preprocessor::TokenMerger;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"NfqhgPyWBObz85VWwk1z9",
    init: |_| {
        Skill::builder()
            .add_preprocessor(TokenMerger::new(NumeralTokenParser))
            .add_corpus(NanoIdCorpus)
            .add_suggestions(NanoIdSuggestions)
            .add_translator(NanoIdTranslator)
            .set_command(NanoIdCommand)
            .build()
    },
};

pub struct NanoIdCorpus;

impl Corpus for NanoIdCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        iproduct!(["generate", "new"], ["nanoid", "NanoID"])
            .flat_map(|(verb, name)| {
                vec![tk!([
                    verb,
                    name; "command"
                ])]
            })
            .chain(
                iproduct!(["generate", "new"], ["nanoid", "NanoID"], 1..=10).flat_map(
                    |(verb, name, count)| {
                        vec![tk!([
                            verb,
                            format!("{}", count); "input:count",
                            name; "command"
                        ])]
                    },
                ),
            )
            .chain(
                iproduct!(
                    ["lower", "upper", "lowercase", "uppercase"],
                    ["alphabet", "number"]
                )
                .flat_map(|(case, kind)| {
                    vec![tk!([
                        "nanoid"; "command",
                        *case; "input:charset",
                        *kind; "input:charset"
                    ])]
                }),
            )
            .chain(["nanoid", "NanoID"].map(|name| tk!([name; "command"])))
            .collect()
    }
}

pub struct NanoIdSuggestions;

impl Suggestions for NanoIdSuggestions {
    fn suggestions(&self, seed: u64, args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        let count = args
            .iter()
            .find(|arg| arg.tag == "input:count")
            .and_then(|arg| arg.as_str().parse::<u8>().ok())
            .filter(|&n| n > 0);
        let mut rng = StdRng::seed_from_u64(seed);
        let n = match count {
            Some(n) => n,
            None if args.is_empty() => rng.gen_range(2..=10),
            _ => return vec!["Generate NanoID".to_string()],
        };
        vec![format!("Generate {} NanoIDs", n)]
    }
}

pub struct NanoIdTranslator;

impl Translator for NanoIdTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        if !args
            .iter()
            .any(|arg| arg.tag == "command" && normalized_eq(arg.as_str(), &["NanoID"], 0))
        {
            return None;
        }

        let lower = args.iter().any(|arg| {
            arg.tag == "input:charset" && normalized_eq(arg.as_str(), &["lower", "lowercase"], 1)
        });
        let upper = args.iter().any(|arg| {
            arg.tag == "input:charset" && normalized_eq(arg.as_str(), &["upper", "uppercase"], 1)
        });
        let alphabet = args
            .iter()
            .any(|arg| arg.tag == "input:charset" && normalized_eq(arg.as_str(), &["alphabet"], 1));
        let number = args
            .iter()
            .any(|arg| arg.tag == "input:charset" && normalized_eq(arg.as_str(), &["number"], 1));

        let charset = if lower && !upper {
            if alphabet && !number {
                "abcdefghijklmnopqrstuvwxyz"
            } else {
                "abcdefghijklmnopqrstuvwxyz0123456789"
            }
        } else if !lower && upper {
            if alphabet && !number {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            } else {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            }
        } else if alphabet && !number {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        } else if !alphabet && number {
            "0123456789"
        } else if alphabet && number {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        } else {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_~"
        };

        let count = args
            .iter()
            .find(|arg| arg.tag == "input:count")
            .and_then(|arg| arg.as_str().parse::<usize>().ok())
            .unwrap_or(1);
        Some(
            CommandArgs::new()
                .add_args(["-n".to_string(), count.to_string()])
                .add_args(["-c", charset]),
        )
    }
}

#[cfg(feature = "wild")]
const MAX_COUNT: usize = u16::MAX as _;

#[cfg(not(feature = "wild"))]
const MAX_COUNT: usize = 32;

pub struct NanoIdCommand;

impl Command for NanoIdCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _user: &UserContext,
    ) -> Result<Output, CommandError> {
        let mut args = Args::try_parse_from(args.args)?;
        if args.n > MAX_COUNT {
            return Err(Output::new()
                .set_title("NanoID Generator")
                .add_block(block::Comment::new().set_text(format!(
                    "Too large number of the requested NanoIDs (Limit: {}).",
                    MAX_COUNT
                )))
                .into());
        }

        let len = args.len;
        let charset = args.charset.drain(..).collect::<Vec<_>>();
        let list = iter::repeat_with(|| nanoid::nanoid!(len, &charset))
            .take(args.n)
            .collect::<Vec<_>>();

        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/nanoid/")?;
        Ok(Output::new()
            .set_title("NanoID Generator")
            .add_blocks_iter(vec![Block::Data(
                block::Data::new().set_text_data(list.join("\n")),
            )])
            .add_metadata(docs))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, default_value_t = 1)]
    pub n: usize,

    #[clap(short, long, default_value_t = 21)]
    pub len: usize,

    #[clap(short, long)]
    pub charset: String,
}
