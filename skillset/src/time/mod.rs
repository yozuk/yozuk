use clap::Parser;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use time_tz::{timezones, Offset, TimeZone};
use yozuk_helper_english::normalized_eq;
use yozuk_helper_platform::time::now_utc;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"DzNEy_nUaQUFmFGqpZZux",
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
            tk!(["now"; "keyword"]),
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

const TIMESTAMP_TOLERANCE_DAYS: i64 = 365 * 10;

#[derive(Debug)]
pub struct TimeTranslator;

impl Translator for TimeTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "keyword")
            .collect::<Vec<_>>();

        if let [time, ..] = keywords[..] {
            if normalized_eq(time.as_str(), &["time", "now"], 0) {
                return Some(CommandArgs::new());
            }
        }

        let timestamps = args
            .iter()
            .filter(|arg| arg.tag == "input:unix")
            .filter_map(|arg| arg.as_str().parse::<i64>().ok())
            .flat_map(|ts| {
                OffsetDateTime::from_unix_timestamp(ts)
                    .ok()
                    .into_iter()
                    .chain(OffsetDateTime::from_unix_timestamp_nanos(ts as i128 * 1000000).ok())
                    .chain(OffsetDateTime::from_unix_timestamp_nanos(ts as i128).ok())
            })
            .filter(|&ts| {
                (now_utc() - ts).whole_days().abs() <= TIMESTAMP_TOLERANCE_DAYS
            })
            .collect::<Vec<_>>();

        if let [ts] = timestamps[..] {
            let ts = ts.unix_timestamp_nanos().to_string();
            return Some(CommandArgs::new().add_args(["--timestamp", &ts]));
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
        i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;

        let tz = i18n
            .timezone
            .as_ref()
            .and_then(|tz| timezones::get_by_name(tz))
            .unwrap_or(timezones::db::UTC);

        if let Some(unix) = args.timestamp {
            let ts = OffsetDateTime::from_unix_timestamp_nanos(unix)?;
            let offset = tz.get_offset_utc(&ts);
            let ts = ts.to_offset(offset.to_utc());
            Ok(Output::new().add_block(block::Data::new().set_text_data(ts.format(&Rfc3339)?)))
        } else {
            let now = now_utc();
            let offset = tz.get_offset_utc(&now);
            let now = now.to_offset(offset.to_utc());
            let text = format!("{}\n{}", now.unix_timestamp(), now.format(&Rfc3339)?);
            Ok(Output::new().add_block(block::Data::new().set_text_data(text)))
        }
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    pub timestamp: Option<i128>,
}
