use clap::Parser;
use itertools::iproduct;
use time::format_description::well_known::{Rfc2822, Rfc3339};
use time::OffsetDateTime;
use time_tz::{timezones, Offset, TimeZone};
use yozuk_helper_english::normalized_eq;
use yozuk_helper_platform::time::now_utc;
use yozuk_sdk::prelude::*;
use yozuk_sdk::preprocessor::{TokenMerger, TokenParser};

mod format;
use format::ENTRIES;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"0YGXydP31~VzDB5ccos2c",
    init: |_| {
        Skill::builder()
            .add_corpus(TimeCorpus)
            .add_preprocessor(TokenMerger::new(TimeTokenParser))
            .add_translator(TimeTranslator)
            .set_command(TimeCommand)
            .build()
    },
};

pub struct TimeCorpus;

impl Corpus for TimeCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        let inputs = [
            "1660440000",
            "3869428800",
            "Sun, 14 Aug 2022 02:00:00 +0000",
            "2022-08-14T02:00:00+00:00",
            "@4000000062f7d8b5",
        ];
        vec![
            tk!(["now"; "keyword"]),
            tk!(["What", "time"; "keyword", "is", "it"]),
            tk!(["What's", "the", "time"; "keyword"]),
            tk!(["What's", "the", "time"; "keyword"]),
            tk!(["current", "time"; "keyword"]),
            tk!(["time"; "keyword"]),
        ]
        .into_iter()
        .chain(
            iproduct!(
                inputs,
                ["as", "to", "in", "into"],
                ENTRIES.iter().flat_map(|entry| entry.keywords)
            )
            .map(|(data, prefix, alg)| {
                tk!([
                    data; "input:data",
                    prefix,
                    *alg; "input:alg"
                ])
            }),
        )
        .collect()
    }
}

const TIMESTAMP_TOLERANCE_DAYS: i64 = 365 * 10;

fn parse_datetime(exp: &str) -> Option<OffsetDateTime> {
    if let Ok(ts) = exp.parse::<i64>() {
        let ts = if let Ok(ts) = OffsetDateTime::from_unix_timestamp(ts) {
            Some(ts)
        } else if let Ok(ts) = OffsetDateTime::from_unix_timestamp_nanos(ts as i128 * 1000000) {
            Some(ts)
        } else if let Ok(ts) = OffsetDateTime::from_unix_timestamp_nanos(ts as i128 * 1000) {
            Some(ts)
        } else if let Ok(ts) = OffsetDateTime::from_unix_timestamp_nanos(ts as i128) {
            Some(ts)
        } else {
            None
        };
        match ts {
            Some(ts) if (now_utc() - ts).whole_days().abs() <= TIMESTAMP_TOLERANCE_DAYS => Some(ts),
            _ => None,
        }
    } else if let Ok(time) = OffsetDateTime::parse(exp, &Rfc3339) {
        Some(time)
    } else if let Ok(time) = OffsetDateTime::parse(exp, &Rfc2822) {
        Some(time)
    } else {
        None
    }
}

struct TimeTokenParser;

impl TokenParser for TimeTokenParser {
    fn parse(&self, tokens: &[Token]) -> Option<Token> {
        let exp = tokens
            .iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let tag = tokens
            .iter()
            .map(|token| token.tag.clone())
            .find(|tag| !tag.is_empty())
            .unwrap_or_default();
        parse_datetime(&exp).map(|_| Token {
            data: exp.into(),
            tag,
            ..Default::default()
        })
    }
}

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
            .filter_map(|arg| parse_datetime(arg.as_str()))
            .collect::<Vec<_>>();

        let algs = args
            .iter()
            .filter(|arg| arg.tag == "input:alg")
            .collect::<Vec<_>>();

        if let [ts] = timestamps[..] {
            let ts = ts.unix_timestamp_nanos().to_string();
            if algs.iter().all(|arg| {
                ENTRIES
                    .iter()
                    .any(|entry| normalized_eq(arg.as_str(), entry.keywords, 0))
            }) {
                return Some(
                    CommandArgs::new()
                        .add_args(["--timestamp", &ts])
                        .add_args_iter(algs.iter().flat_map(|arg| ["--format", arg.as_str()])),
                );
            } else {
                return Some(CommandArgs::new().add_args(["--timestamp", &ts]));
            }
        }

        None
    }
}

pub struct TimeCommand;

impl Command for TimeCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        user: &UserContext,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;

        let tz = user
            .timezone
            .as_ref()
            .and_then(|tz| timezones::get_by_name(tz))
            .unwrap_or(timezones::db::UTC);

        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/time/")?;
        let ts = if let Some(unix) = args.timestamp {
            let ts = OffsetDateTime::from_unix_timestamp_nanos(unix)?;
            let offset = tz.get_offset_utc(&ts);
            ts.to_offset(offset.to_utc())
        } else {
            let now = now_utc();
            let offset = tz.get_offset_utc(&now);
            now.to_offset(offset.to_utc())
        };

        let mut entries = Vec::new();
        for name in &args.format {
            let matched = ENTRIES
                .iter()
                .filter(|entry| normalized_eq(name, entry.keywords, 0))
                .collect::<Vec<_>>();

            if matched.is_empty() {
                return Err(Output::new()
                    .set_title("Timestamp Converter")
                    .add_block(
                        block::Comment::new().set_text(format!("Unsupprted format: {}", name)),
                    )
                    .into());
            }

            for entry in matched {
                entries.push(entry);
            }
        }

        let formats = if entries.is_empty() {
            let unix = ts.unix_timestamp();
            vec![unix.to_string()]
                .into_iter()
                .chain(ts.format(&Rfc2822).ok())
                .chain(ts.format(&Rfc3339).ok())
                .collect::<Vec<_>>()
        } else {
            entries
                .into_iter()
                .flat_map(|entry| (entry.format)(&ts))
                .collect::<Vec<_>>()
        };

        Ok(Output::new()
            .set_title("Timestamp Converter")
            .add_block(
                block::Data::new()
                    .set_highlighted_text_data(formats.join("\n"), &Default::default()),
            )
            .add_metadata(docs))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, multiple_occurrences(true))]
    pub format: Vec<String>,
    #[clap(short, long, allow_hyphen_values = true)]
    pub timestamp: Option<i128>,
}
