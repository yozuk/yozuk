#![forbid(unsafe_code)]
#![deny(clippy::all)]

use bech32::{ToBase32, Variant};
use clap::{ArgEnum, Parser};
use itertools::iproduct;
use std::io::Read;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;
use yozuk_sdk::Bytes;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"4Bx-A7Mi9opU6AxMyROyi",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(Bech32Corpus)
            .add_translator(Bech32Translator)
            .set_command(Bech32Command)
            .build()
    },
};

#[derive(Debug)]
pub struct Bech32Corpus;

impl Corpus for Bech32Corpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        let inputs = vec![
            "Hello World",
            "😍😗😋",
            "quick brown fox jumps over the lazy dog",
            "Veterinarian",
        ];
        iproduct!(
            inputs.clone(),
            ["Bech32", "Bech32m"],
            ["as", "to", "in", "into"]
        )
        .map(|(data, alg, prefix)| {
            tk!([
                data; "input:data",
                prefix,
                alg; "command:bech32"
            ])
        })
        .chain(
            iproduct!(inputs, ["Bech32", "Bech32m"], ["of"]).map(|(data, alg, suffix)| {
                tk!([
                    alg; "command:bech32",
                    suffix,
                    data; "input:data"
                ])
            }),
        )
        .collect()
    }
}

#[derive(Debug)]
pub struct Bech32Translator;

impl Translator for Bech32Translator {
    fn parse(&self, args: &[Token], streams: &[InputStream]) -> Option<CommandArgs> {
        let input = args
            .iter()
            .filter(|arg| arg.tag == "input:data")
            .map(|arg| arg.data.clone())
            .collect::<Vec<_>>();

        if !input.is_empty() || !streams.is_empty() {
            if args.iter().any(|arg| {
                arg.tag == "command:bech32" && normalized_eq(arg.as_str(), &["Bech32"], 0)
            }) {
                return Some(
                    CommandArgs::new()
                        .add_args(["--mode", "encode-bech32"])
                        .add_data_iter(input),
                );
            }

            if args.iter().any(|arg| {
                arg.tag == "command:bech32" && normalized_eq(arg.as_str(), &["Bech32m"], 0)
            }) {
                return Some(
                    CommandArgs::new()
                        .add_args(["--mode", "encode-bech32m"])
                        .add_data_iter(input),
                );
            }
        }

        let is_bech32 = args.iter().all(|arg| bech32::decode(arg.as_str()).is_ok());
        if is_bech32 {
            return Some(
                CommandArgs::new()
                    .add_args(["--mode", "decode"])
                    .add_args_iter(args.iter().map(|arg| arg.as_str())),
            );
        }
        None
    }
}

#[derive(Debug)]
pub struct Bech32Command;

impl Command for Bech32Command {
    fn run(
        &self,
        args: CommandArgs,
        streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let options = Args::try_parse_from(args.args)?;

        match options.mode {
            Mode::Decode => {
                let blocks = options
                    .inputs
                    .iter()
                    .filter_map(|arg| bech32::decode(arg).ok())
                    .flat_map(|(hrp, data, variant)| {
                        let data = bech32::convert_bits(&data, 5, 8, true).unwrap();
                        let variant = match variant {
                            Variant::Bech32 => "Bech32",
                            Variant::Bech32m => "Bech32m",
                        };
                        vec![
                            Block::Comment(
                                block::Comment::new().set_text(format!("Decoding {}", variant)),
                            ),
                            Block::Data(block::Data::new().set_text_data(hrp)),
                            Block::Data(block::Data::new().set_data(data)),
                        ]
                    });
                Ok(Output::new().set_title("Bech32 Decoder").add_blocks(blocks))
            }
            _ => {
                let (variant, name) = if options.mode == Mode::EncodeBech32 {
                    (Variant::Bech32, "Bech32")
                } else {
                    (Variant::Bech32m, "Bech32m")
                };
                let streams = streams.iter_mut().map(|stream| {
                    stream
                        .bytes()
                        .map(|b| b.unwrap_or_default())
                        .collect::<Bytes>()
                });
                let streams = args.data.into_iter().chain(streams);
                let blocks = streams
                    .into_iter()
                    .filter_map(|data| bech32::encode("b32", data.to_base32(), variant).ok())
                    .map(|data| Block::Data(block::Data::new().set_text_data(data)));
                Ok(Output::new()
                    .set_title(format!("{name} Encoder"))
                    .add_blocks(blocks))
            }
        }
    }

    fn priority(&self) -> i32 {
        -100
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,

    #[clap(arg_enum, short, long)]
    pub mode: Mode,
}

#[derive(ArgEnum, Clone, PartialEq, Eq)]
enum Mode {
    Decode,
    EncodeBech32,
    EncodeBech32m,
}
