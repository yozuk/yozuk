use clap::Parser;
use time::format_description::well_known::{Rfc2822, Rfc3339};
use time::OffsetDateTime;
use time_tz::{timezones, Offset, TimeZone};
use yozuk_helper_english::normalized_eq;
use yozuk_helper_platform::time::now_utc;
use yozuk_sdk::prelude::*;
use yozuk_sdk::preprocessor::{TokenMerger, TokenParser};

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"DzNEy_nUaQUFmFGqpZZux",
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
        OffsetDateTime::parse(&exp, &Rfc2822).ok().map(|_| Token {
            data: exp.into(),
            tag,
            ..Default::default()
        })
    }
}

const TIMESTAMP_TOLERANCE_DAYS: i64 = 365 * 10;

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
            .filter(|&ts| (now_utc() - ts).whole_days().abs() <= TIMESTAMP_TOLERANCE_DAYS)
            .chain(
                args.iter()
                    .filter_map(|arg| OffsetDateTime::parse(arg.as_str(), &Rfc3339).ok()),
            )
            .chain(
                args.iter()
                    .filter_map(|arg| OffsetDateTime::parse(arg.as_str(), &Rfc2822).ok()),
            )
            .collect::<Vec<_>>();

        if let [ts] = timestamps[..] {
            let ts = ts.unix_timestamp_nanos().to_string();
            return Some(CommandArgs::new().add_args(["--timestamp", &ts]));
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
        i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;

        let tz = i18n
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

        let unix = ts.unix_timestamp();
        let mut formats = vec![
            format!("unix: `{}`", unix),
            format!("ntp: `{}`", unix + NTP_OFFSET),
            format!("rfc2822: `{}`", ts.format(&Rfc2822)?),
            format!("rfc3339: `{}`", ts.format(&Rfc3339)?),
        ];
        if let Some(tai64) = unix_to_tai64(unix) {
            formats.push(format!(
                "tai64: `@{}`",
                hex::encode((tai64 + TAI64_LABEL_OFFSET).to_be_bytes())
            ));
        }

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
    #[clap(short, long, allow_hyphen_values = true)]
    pub timestamp: Option<i128>,
}

const NTP_OFFSET: i64 = 2208988800;
const TAI64_LABEL_OFFSET: u64 = 0x4000000000000000 - NTP_OFFSET as u64;

fn unix_to_tai64(unix: i64) -> Option<u64> {
    let ntp = (unix + NTP_OFFSET) as u64;
    let index = match TAI64_LEAP_SECONDS.binary_search_by_key(&ntp, |(ts, _)| *ts) {
        Ok(n) => n,
        Err(n) => n,
    };
    TAI64_LEAP_SECONDS.get(index).map(|(_, leap)| ntp + leap)
}

// https://www.ietf.org/timezones/data/leap-seconds.list
const TAI64_LEAP_SECONDS: &[(u64, u64)] = &[
    (2272060800, 10),
    (2287785600, 11),
    (2303683200, 12),
    (2335219200, 13),
    (2366755200, 14),
    (2398291200, 15),
    (2429913600, 16),
    (2461449600, 17),
    (2492985600, 18),
    (2524521600, 19),
    (2571782400, 20),
    (2603318400, 21),
    (2634854400, 22),
    (2698012800, 23),
    (2776982400, 24),
    (2840140800, 25),
    (2871676800, 26),
    (2918937600, 27),
    (2950473600, 28),
    (2982009600, 29),
    (3029443200, 30),
    (3076704000, 31),
    (3124137600, 32),
    (3345062400, 33),
    (3439756800, 34),
    (3550089600, 35),
    (3644697600, 36),
    (3692217600, 37),
    (3881174400 - 1, 37),
];
