#![forbid(unsafe_code)]
#![deny(clippy::all)]

use chrono::prelude::*;
use chrono::SecondsFormat;
use mediatype::media_type;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"RivT_4cXJtYS1h5JpMxdxd",
    config_schema: None,
    init: |env, _| {
        Skill::builder()
            .add_corpus(TimeCorpus)
            .add_translator(TimeTranslator)
            .set_command(TimeCommand(env.clone()))
            .build()
    },
};

#[derive(Debug)]
pub struct TimeCorpus;

impl Corpus for TimeCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!(["now"; "time:keyword"]),
            tk!(["now"; "time:keyword"]),
            tk!(["now"; "time:keyword"]),
            tk!(["now"; "time:keyword"]),
            tk!(["now"; "time:keyword"]),
            tk!(["now"; "time:keyword"]),
            tk!(["now"; "time:keyword"]),
            tk!(["What", "time"; "time:keyword", "is", "it"]),
            tk!(["What's", "the", "time"; "time:keyword"]),
            tk!(["current", "time"; "time:keyword"]),
            tk!(["time"; "time:keyword"]),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Debug)]
pub struct TimeTranslator;

impl Translator for TimeTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "time:keyword")
            .collect::<Vec<_>>();

        if let [time, ..] = keywords[..] {
            if normalized_eq(time.as_utf8(), &["time", "now"], 0) {
                return Some(CommandArgs::new());
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct TimeCommand(Environment);

impl Command for TimeCommand {
    fn run(
        &self,
        _args: CommandArgs,
        _streams: &mut [InputStream],
    ) -> Result<Output, CommandError> {
        let now = Local::now();
        let time = format!(
            "{}\n{}",
            now.timestamp(),
            now.to_rfc3339_opts(SecondsFormat::Millis, false)
        );
        Ok(Output {
            module: "Time".into(),
            sections: vec![Section::new(time, media_type!(TEXT / PLAIN))],
        })
    }
}
