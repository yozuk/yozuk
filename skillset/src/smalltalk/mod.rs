#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use mediatype::media_type;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"CI5cJBeuNmRj5mMRVAC_X",
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
            if normalized_eq(life.as_utf8(), &["life"], 1)
                && normalized_eq(universe.as_utf8(), &["universe"], 1)
                && normalized_eq(everything.as_utf8(), &["everything"], 1)
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
    fn run(&self, args: CommandArgs, _streams: &mut [InputStream]) -> Result<Output, Output> {
        let args = Args::try_parse_from(args.args).unwrap();
        if args.life_universe_everything {
            Ok(Output {
                module: "Deep Thought".into(),
                sections: vec![Section::new(
                "Computing the answer to your question will take a little while. Please ask me \
                 again seven and a half million years later.",
                media_type!(TEXT / PLAIN),
            )
            .kind(SectionKind::Comment)],
            })
        } else {
            Ok(Output {
                sections: vec![Section::new("Hi. I'm Yozuk.", media_type!(TEXT / PLAIN))
                    .kind(SectionKind::Comment)],
                ..Default::default()
            })
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
