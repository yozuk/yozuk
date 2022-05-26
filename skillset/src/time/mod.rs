use clap::Parser;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"DzNEy_nUaQUFmFGqpZZun",
    init: |_| {
        Skill::builder()
            .add_corpus(TimeCorpus)
            .add_translator(TimeTranslator)
            .set_command(TimeCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct TimeCorpus;

impl Corpus for TimeCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!(["What", "time"; "keyword", "is", "it"]),
            tk!(["What's", "the", "time"; "keyword"]),
            tk!(["current", "time"; "keyword"]),
            tk!(["time"; "keyword"]),
            tk!(["1640000000"; "input:unix"]),
            tk!(["1640000000000"; "input:unix"]),
            tk!(["1640000000000000000"; "input:unix"]),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Debug)]
pub struct TimeTranslator;

impl Translator for TimeTranslator {
    fn parse(&self, args: &[Token], streams: &[InputStream]) -> Option<CommandArgs> {
        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "keyword")
            .collect::<Vec<_>>();

        if let [time, ..] = keywords[..] {
            if normalized_eq(time.as_str(), &["time", "now"], 0) {
                return Some(CommandArgs::new());
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct TimeCommand;

impl Command for TimeCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let _args = Args::try_parse_from(args.args)?;
        Ok(Output::new().add_block(block::Comment::new().set_text("Hi. I'm Yozuk.")))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    pub timestamp: Option<i64>,

    #[clap(short, long)]
    pub exp: Option<String>,
}
