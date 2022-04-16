#![forbid(unsafe_code)]
#![deny(clippy::all)]

use chrono::prelude::*;
use chrono::SecondsFormat;
use chrono_tz::Tz;
use clap::Parser;
use mediatype::media_type;
use yozuk_helper_english::normalized_eq;
use yozuk_helper_preprocessor::{TokenMerger, TokenParser};
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"flWuS_TMghY1Q5wq4fQ_qc",
    config_schema: None,
    init: |env, _| {
        Skill::builder()
            .add_corpus(TimeCorpus)
            .add_preprocessor(TokenMerger::new(TimeTokenParser))
            .add_translator(TimeTranslator)
            .set_command(TimeCommand(env.clone()))
            .build()
    },
};

const TIMESTAMP_TOLERANCE_DAYS: i64 = 365 * 10;

#[derive(Debug)]
pub struct TimeCorpus;

impl Corpus for TimeCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!(["What", "time"; "time:keyword", "is", "it"]),
            tk!(["What's", "the", "time"; "time:keyword"]),
            tk!(["current", "time"; "time:keyword"]),
            tk!(["time"; "time:keyword"]),
            tk!(["1640000000"; "input:unix"]),
            tk!(["1640000000000"; "input:unix"]),
            tk!(["1640000000000000000"; "input:unix"]),
        ]
        .into_iter()
        .collect()
    }
}

struct TimeTokenParser;

impl TokenParser for TimeTokenParser {
    fn parse(&self, tokens: &[Token]) -> Option<Token> {
        let exp = tokens
            .iter()
            .map(|token| token.as_utf8())
            .collect::<Vec<_>>()
            .join(" ");

        fuzzydate::parse(&exp).ok().map(|_| tk!(exp; "input:exp"))
    }
}

#[derive(Debug)]
pub struct TimeTranslator;

impl Translator for TimeTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let exps = args
            .iter()
            .filter(|arg| arg.tag == "input:exp")
            .collect::<Vec<_>>();

        if let [exp] = exps[..] {
            if fuzzydate::parse(exp.as_utf8()).is_ok() {
                return Some(CommandArgs::new().add_args(["--exp", exp.as_utf8()]));
            }
        }

        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "time:keyword")
            .collect::<Vec<_>>();

        if let [time, ..] = keywords[..] {
            if normalized_eq(time.as_utf8(), &["time", "now"], 0) {
                return Some(CommandArgs::new());
            }
        }

        let timestamps = args
            .iter()
            .filter(|arg| arg.tag == "input:unix")
            .filter_map(|arg| arg.as_utf8().parse::<i64>().ok())
            .flat_map(|ts| {
                Utc.timestamp_millis_opt(ts)
                    .single()
                    .into_iter()
                    .chain(Utc.timestamp_opt(ts, 0).single())
                    .chain(Some(Utc.timestamp_nanos(ts)))
            })
            .filter(|&ts| (Utc::now() - ts).num_days().abs() <= TIMESTAMP_TOLERANCE_DAYS)
            .collect::<Vec<_>>();

        if let [ts] = timestamps[..] {
            let ts = ts.timestamp_nanos().to_string();
            return Some(CommandArgs::new().add_args(["--timestamp", &ts]));
        }

        None
    }
}

#[derive(Debug)]
pub struct TimeCommand(Environment);

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
            .and_then(|tz| tz.parse().ok())
            .unwrap_or(Tz::UTC);

        let time = if let Some(ts) = args.timestamp {
            let ts = Utc.timestamp_nanos(ts);
            vec![ts.to_rfc3339_opts(SecondsFormat::Millis, false)]
        } else if let Some(ts) = args.exp {
            let ts = fuzzydate::parse(&ts).unwrap_or_else(|_| Local::now().naive_local());
            let ts = tz
                .from_local_datetime(&ts)
                .single()
                .unwrap_or_else(|| tz.from_utc_datetime(&Utc::now().naive_utc()));
            vec![
                ts.timestamp().to_string(),
                ts.to_rfc3339_opts(SecondsFormat::Millis, false),
            ]
        } else {
            let now = Local::now();
            vec![
                now.timestamp().to_string(),
                now.to_rfc3339_opts(SecondsFormat::Millis, false),
            ]
        };
        Ok(Output {
            module: "Time".into(),
            sections: vec![Section::new(time.join("\n"), media_type!(TEXT / PLAIN))],
        })
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    pub timestamp: Option<i64>,

    #[clap(short, long)]
    pub exp: Option<String>,
}
