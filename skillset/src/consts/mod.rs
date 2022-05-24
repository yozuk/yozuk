use clap::Parser;
use yozuk_sdk::prelude::*;

mod definition;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"bQ49wkMRKLOjoZ17U0-i9",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(ConstCorpus)
            .add_translator(ConstTranslator)
            .set_command(ConstCommand)
            .build()
    },
};

#[derive(Default)]
struct Constant {
    pub name: &'static str,
    pub tokens: Vec<Vec<Token>>,
    pub value: &'static str,
    pub unit: Option<&'static str>,
}

#[derive(Debug)]
pub struct ConstCorpus;

impl Corpus for ConstCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!([
                "Life"; "keyword",
                "universe"; "keyword",
                "everything"; "keyword"
            ]),
            tk!([
                "Life,"; "keyword",
                "the",
                "universe"; "keyword",
                "and",
                "everything"; "keyword"
            ]),
            tk!([
                "The", "answer", "to",
                "Life,"; "keyword",
                "universe"; "keyword",
                "and",
                "everything"; "keyword"
            ]),
            tk!([
                "The", "answer", "to",
                "Life,"; "keyword",
                "universe"; "keyword",
                "and",
                "everything"; "keyword"
            ]),
            tk!([
                "The", "answer", "to",
                "Life,"; "keyword",
                "the",
                "universe"; "keyword",
                "and",
                "everything"; "keyword"
            ]),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Debug)]
pub struct ConstTranslator;

impl Translator for ConstTranslator {
    fn parse(&self, _args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        None
    }
}

#[derive(Debug)]
pub struct ConstCommand;

impl Command for ConstCommand {
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
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub life_universe_everything: bool,
}
