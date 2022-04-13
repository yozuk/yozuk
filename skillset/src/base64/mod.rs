#![forbid(unsafe_code)]
#![deny(clippy::all)]

use chardetng::EncodingDetector;
use clap::{ArgEnum, Parser};
use itertools::iproduct;
use mediatype::{
    media_type,
    names::{CHARSET, OCTET_STREAM, PLAIN, TEXT},
    MediaType, Value, WriteParams,
};
use std::io::Read;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;
use yozuk_sdk::Bytes;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"-6zq-7vR7Ax6tHYBelhCl",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_labeler(Base64Labeler)
            .add_corpus(Base64Corpus)
            .add_translator(Base64Translator)
            .set_command(Base64Command)
            .build()
    },
};

const MINIMUM_SHANNON_ENTROPY: f32 = 2.5;

fn label_base64(token: &Token) -> impl Iterator<Item = Feature> {
    Some(token)
        .filter(|token| token.shannon_entropy() >= MINIMUM_SHANNON_ENTROPY)
        .and_then(|token| base64::decode(&token.data).ok())
        .map(|_| Feature {
            name: "encoding:base64".into(),
            non_entity: true,
            ..Default::default()
        })
        .into_iter()
}

#[derive(Debug)]
pub struct Base64Corpus;

impl Corpus for Base64Corpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        let inputs = vec![
            "Hello World",
            "😍😗😋",
            "quick brown fox jumps over the lazy dog",
            "Veterinarian",
        ];
        iproduct!(inputs.clone(), ["as", "to", "in", "into"])
            .map(|(data, prefix)| {
                tk!([
                    data; "input:data",
                    prefix,
                    "Base64"; "command:base64"
                ])
            })
            .chain(
                iproduct!(inputs, ["of", "of", "of", "of"]).map(|(data, suffix)| {
                    tk!([
                        "Base64"; "command:base64",
                        suffix,
                        data; "input:data"
                    ])
                }),
            )
            .chain(vec![tk!(["Base64"; "command:base64"]); 10])
            .chain(vec![tk!(["SGVsbG8gV29ybGQh"; "input:base64"]); 10])
            .collect()
    }
}

#[derive(Debug)]
pub struct Base64Labeler;

impl Labeler for Base64Labeler {
    fn label_features(&self, input: &[Token]) -> Vec<Vec<Feature>> {
        input
            .iter()
            .map(|token| label_base64(token).collect())
            .collect()
    }
}

#[derive(Debug)]
pub struct Base64Translator;

impl Translator for Base64Translator {
    fn parse(&self, args: &[Token], streams: &[InputStream]) -> Option<CommandArgs> {
        if args
            .iter()
            .any(|arg| arg.tag == "command:base64" && normalized_eq(arg.as_utf8(), &["Base64"], 0))
        {
            let input = args
                .iter()
                .filter(|arg| arg.tag == "input:data")
                .map(|arg| arg.data.clone())
                .collect::<Vec<_>>();

            if !input.is_empty() || !streams.is_empty() {
                return Some(
                    CommandArgs::new()
                        .add_args(["--mode", "encode"])
                        .add_data_iter(input),
                );
            }
        }

        let inputs = args
            .iter()
            .filter(|arg| arg.tag == "input:base64")
            .collect::<Vec<_>>();
        if inputs.len() > 1 {
            return None;
        }

        let input = inputs
            .into_iter()
            .filter(|arg| {
                if base64::decode(&arg.as_utf8()).is_ok() {
                    arg.shannon_entropy() >= MINIMUM_SHANNON_ENTROPY
                } else {
                    false
                }
            })
            .map(|arg| arg.data.clone())
            .collect::<Vec<_>>();

        if !input.is_empty()
            || (!streams.is_empty()
                && streams
                    .iter()
                    .all(|stream| !stream.header().is_empty() && stream.header().is_ascii()))
        {
            return Some(
                CommandArgs::new()
                    .add_args(["--mode", "decode"])
                    .add_data_iter(input),
            );
        }

        None
    }
}

#[derive(Debug)]
pub struct Base64Command;

impl Command for Base64Command {
    fn run(
        &self,
        args: CommandArgs,
        streams: &mut [InputStream],
        _locale: &Locale,
    ) -> Result<Output, CommandError> {
        let streams = streams.iter_mut().map(|stream| {
            stream
                .bytes()
                .map(|b| b.unwrap_or_default())
                .collect::<Bytes>()
        });
        let options = Options::try_parse_from(args.args)?;
        match options.mode {
            Mode::Decode => {
                let mut sections = vec![Section::new(
                    "Decoding Base64 string".to_string(),
                    media_type!(TEXT / PLAIN),
                )
                .kind(SectionKind::Comment)];

                sections.append(
                    &mut args
                        .data
                        .into_iter()
                        .chain(streams)
                        .filter_map(|data| base64::decode(&data).ok())
                        .map(|data| {
                            let media_type = tree_magic::from_u8(&data);
                            let mut encdetector = EncodingDetector::new();
                            encdetector.feed(&data, true);

                            let mut media_type = MediaType::parse(&media_type).unwrap();
                            if media_type.ty == TEXT {
                                let enc = encdetector.guess(None, true);
                                media_type.set_param(CHARSET, Value::new(enc.name()).unwrap());
                            } else if media_type.subty == OCTET_STREAM {
                                let (enc, likely) = encdetector.guess_assess(None, true);
                                if likely {
                                    media_type.ty = TEXT;
                                    media_type.subty = PLAIN;
                                    media_type.set_param(CHARSET, Value::new(enc.name()).unwrap());
                                }
                            }

                            Section::new(data, media_type)
                        })
                        .collect(),
                );

                Ok(Output {
                    module: "Base64 Decoder".into(),
                    sections,
                })
            }
            Mode::Encode => Ok(Output {
                module: "Base64 Encoder".into(),
                sections: args
                    .data
                    .into_iter()
                    .chain(streams)
                    .map(|data| Section::new(base64::encode(data), media_type!(TEXT / PLAIN)))
                    .collect(),
            }),
        }
    }

    fn priority(&self) -> i32 {
        -100
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Options {
    #[clap(arg_enum, short, long)]
    pub mode: Mode,
}

#[derive(ArgEnum, Clone)]
enum Mode {
    Decode,
    Encode,
}
