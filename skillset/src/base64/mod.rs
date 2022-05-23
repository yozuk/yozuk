use chardetng::EncodingDetector;
use clap::{ArgEnum, Parser};
use itertools::iproduct;
use mediatype::{
    media_type,
    names::{CHARSET, PLAIN, TEXT},
    Value, WriteParams,
};
use std::io::Read;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;
use yozuk_sdk::Bytes;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"u4q_8wwjC8oi7rZFUs0U4",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(Base64Corpus)
            .add_translator(Base64Translator)
            .set_command(Base64Command)
            .build()
    },
};

const MINIMUM_ENTROPY_SCORE: f32 = 10.0;

fn is_like_base64(data: &[u8]) -> bool {
    if let Ok(decoded) = base64::decode(data) {
        let score = entropy::shannon_entropy(&data) / entropy::shannon_entropy(&decoded)
            * data.len() as f32;
        return score >= MINIMUM_ENTROPY_SCORE;
    }
    false
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
                    "Base64"; "command"
                ])
            })
            .chain(iproduct!(inputs, ["of"]).map(|(data, suffix)| {
                tk!([
                    "Base64"; "command",
                    suffix,
                    data; "input:data"
                ])
            }))
            .collect()
    }
}

#[derive(Debug)]
pub struct Base64Translator;

impl Translator for Base64Translator {
    fn parse(&self, args: &[Token], streams: &[InputStream]) -> Option<CommandArgs> {
        if args
            .iter()
            .any(|arg| arg.tag == "command" && normalized_eq(arg.as_str(), &["Base64"], 0))
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

        let inputs = args.iter().map(|arg| arg.data.clone()).collect::<Vec<_>>();
        let is_base64 = inputs.len() == 1 && inputs.iter().all(|arg| is_like_base64(arg));
        if is_base64
            || (!streams.is_empty()
                && streams
                    .iter()
                    .all(|stream| !stream.header().is_empty() && stream.header().is_ascii()))
        {
            return Some(
                CommandArgs::new()
                    .add_args(["--mode", "decode"])
                    .add_data_iter(inputs),
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
        _i18n: &I18n,
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
                let mut blocks = vec![Block::Comment(
                    block::Comment::new().set_text("Decoding Base64 string"),
                )];

                blocks.append(
                    &mut args
                        .data
                        .into_iter()
                        .chain(streams)
                        .filter_map(|data| base64::decode(&data).ok())
                        .map(|data| {
                            let mut encdetector = EncodingDetector::new();
                            encdetector.feed(&data, true);
                            let (enc, likely) = encdetector.guess_assess(None, true);

                            let mut media_type = media_type!(APPLICATION / OCTET_STREAM);
                            if likely {
                                media_type.ty = TEXT;
                                media_type.subty = PLAIN;
                                media_type.set_param(CHARSET, Value::new(enc.name()).unwrap());
                            }

                            Block::Data(
                                block::Data::new().set_data(data).set_media_type(media_type),
                            )
                        })
                        .collect(),
                );

                Ok(Output::new()
                    .set_title("Base64 Decoder")
                    .add_blocks_iter(blocks))
            }
            Mode::Encode => Ok(Output::new().set_title("Base64 Encoder").add_blocks_iter(
                args.data
                    .into_iter()
                    .chain(streams)
                    .map(|data| block::Data::new().set_text_data(base64::encode(data))),
            )),
        }
    }

    fn priority(&self) -> i32 {
        -120
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
