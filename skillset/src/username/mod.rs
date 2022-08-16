use clap::Parser;
use itertools::iproduct;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::iter;
use yozuk_helper_english::{normalized_eq, NumeralTokenParser};
use yozuk_sdk::prelude::*;
use yozuk_sdk::preprocessor::TokenMerger;

mod dictionary;
use dictionary::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"0T4ltRczKmeGFfFHz6q0i",
    init: |_| {
        Skill::builder()
            .add_preprocessor(TokenMerger::new(NumeralTokenParser))
            .add_corpus(UsernameCorpus)
            .add_suggestions(UsernameSuggestions)
            .add_translator(UsernameTranslator)
            .set_command(UsernameCommand)
            .build()
    },
};

pub struct UsernameCorpus;

impl Corpus for UsernameCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        iproduct!(
            ["generate", "new"],
            ["username"],
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
            iproduct!(["generate", "new"], ["username"], 1..=10).flat_map(|(verb, name, count)| {
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
            }),
        )
        .chain(
            iproduct!(
                ["generate", "new"],
                ["username"],
                ["lower", "upper", "lowercase", "uppercase",],
                1..=10
            )
            .flat_map(|(verb, name, case, count)| {
                vec![
                    tk!([
                        verb,
                        format!("{}", count); "input:count",
                        name; "command",
                        case; "input:charset"
                    ]),
                    tk!([
                        "please",
                        verb,
                        format!("{}", count); "input:count",
                        name; "command",
                        case; "input:charset"
                    ]),
                ]
            }),
        )
        .chain(["username"].map(|name| tk!([name; "command"])))
        .collect()
    }
}

pub struct UsernameSuggestions;

impl Suggestions for UsernameSuggestions {
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
            _ => return vec!["Generate Username".to_string()],
        };
        vec![format!("Generate {} Usernames", n)]
    }
}

pub struct UsernameTranslator;

impl Translator for UsernameTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        if !args
            .iter()
            .any(|arg| arg.tag == "command" && normalized_eq(arg.as_str(), &["username"], 0))
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

pub struct UsernameCommand;

impl Command for UsernameCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _user: &UserContext,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/username/")?;
        if args.n > MAX_COUNT {
            return Err(Output::new()
                .set_title("Username Generator")
                .add_block(block::Comment::new().set_text(format!(
                    "Too large number of the requested usernames (Limit: {}).",
                    MAX_COUNT
                )))
                .add_metadata(docs)
                .into());
        }
        let mut rng = rand::thread_rng();
        let list = iter::repeat_with(|| {
            let adj = ADJECTIVES.choose(&mut rng).unwrap();
            let noun = NOUNS.choose(&mut rng).unwrap();
            let number = rng.gen_range(1..=9999);
            format!("{adj}{noun}{number}")
        })
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
            .set_title("Username Generator")
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

    #[clap(long)]
    pub upper: bool,
}
