#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"bQ49wkMRKLOjoZ17U0-i9",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(SmalltalkCorpus)
            .add_translator(SmalltalkTranslator)
            .set_command(SmalltalkCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct SmalltalkCorpus;

impl Corpus for SmalltalkCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!([
                "Life"; "smalltalk:keyword",
                "universe"; "smalltalk:keyword",
                "everything"; "smalltalk:keyword"
            ]),
            tk!([
                "Life,"; "smalltalk:keyword",
                "the",
                "universe"; "smalltalk:keyword",
                "and",
                "everything"; "smalltalk:keyword"
            ]),
            tk!([
                "The", "answer", "to",
                "Life,"; "smalltalk:keyword",
                "universe"; "smalltalk:keyword",
                "and",
                "everything"; "smalltalk:keyword"
            ]),
            tk!([
                "The", "answer", "to",
                "Life,"; "smalltalk:keyword",
                "universe"; "smalltalk:keyword",
                "and",
                "everything"; "smalltalk:keyword"
            ]),
            tk!([
                "The", "answer", "to",
                "Life,"; "smalltalk:keyword",
                "the",
                "universe"; "smalltalk:keyword",
                "and",
                "everything"; "smalltalk:keyword"
            ]),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Debug)]
pub struct SmalltalkTranslator;

impl Translator for SmalltalkTranslator {
    fn parse(&self, args: &[Token], streams: &[InputStream]) -> Option<CommandArgs> {
        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "smalltalk:keyword")
            .collect::<Vec<_>>();
        if let [life, universe, everything] = keywords[..] {
            if normalized_eq(life.as_str(), &["life"], 1)
                && normalized_eq(universe.as_str(), &["universe"], 1)
                && normalized_eq(everything.as_str(), &["everything"], 1)
            {
                return Some(CommandArgs::new().add_args(["--life-universe-everything"]));
            }
        }

        if args.is_empty() && streams.is_empty() {
            return Some(CommandArgs::new());
        }

        None
    }
}

#[derive(Debug)]
pub struct SmalltalkCommand;

impl Command for SmalltalkCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        if args.life_universe_everything {
            Ok(Output::new()
                .set_title("Deep Thought")
                .add_block(block::Comment::new().set_text(
                "Computing the answer to your question will take a little while. Please ask me \
                 again seven and a half million years later.",
            )))
        } else {
            Ok(Output::new().add_block(block::Comment::new().set_text("Hi. I'm Yozuk.")))
        }
    }

    fn priority(&self) -> i32 {
        -50
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub life_universe_everything: bool,
}
