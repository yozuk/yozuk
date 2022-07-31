use clap::{ArgEnum, Parser};
use itertools::iproduct;
use std::io::Read;
use yozuk_helper_encoding::EncodingPreprocessor;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::encoding::RawEncoding;
use yozuk_sdk::prelude::*;
use yozuk_sdk::Bytes;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"fdU0PXy1uNGYlypwQiiOK",
    init: |_| {
        Skill::builder()
            .add_corpus(HexCorpus)
            .add_preprocessor(EncodingPreprocessor::new([RawEncoding::Hex]))
            .add_suggestions(HexSuggestions)
            .add_translator(HexTranslator)
            .set_command(HexCommand)
            .build()
    },
};

pub struct HexSuggestions;

impl Suggestions for HexSuggestions {
    fn suggestions(&self, _seed: u64, args: &[Token], streams: &[InputStream]) -> Vec<String> {
        let inputs = args
            .iter()
            .filter(|arg| arg.tag == "input:data")
            .map(|arg| arg.as_str())
            .collect::<Vec<_>>();
        if !inputs.is_empty() {
            vec![format!("{} to hex", shell_words::join(inputs))]
        } else if !streams.is_empty() {
            vec!["hex".to_string()]
        } else {
            vec![]
        }
    }
}

pub struct HexCorpus;

impl Corpus for HexCorpus {
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
                    "hex"; "command"
                ])
            })
            .chain(iproduct!(inputs, ["of"]).map(|(data, suffix)| {
                tk!([
                    "hex"; "command",
                    suffix,
                    data; "input:data"
                ])
            }))
            .collect()
    }
}

pub struct HexTranslator;

impl Translator for HexTranslator {
    fn generate_command(&self, args: &[Token], streams: &[InputStream]) -> Option<CommandArgs> {
        if args
            .iter()
            .any(|arg| arg.tag == "command" && normalized_eq(arg.as_str(), &["hex"], 0))
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
            .filter(|arg| arg.raw_encoding.is_some())
            .map(|arg| hex::encode(&arg.data))
            .collect::<Vec<_>>();
        let is_hex = inputs.len() == 1;
        if is_hex
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

pub struct HexCommand;

impl Command for HexCommand {
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
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/hex/")?;
        match options.mode {
            Mode::Decode => {
                let mut blocks = vec![];

                blocks.append(
                    &mut args
                        .data
                        .into_iter()
                        .chain(streams)
                        .filter_map(|data| hex::decode(&data).ok())
                        .map(|data| {
                            let media_type = yozuk_helper_filetype::guess_media_type(&data);
                            Block::Data(
                                block::Data::new()
                                    .set_data(data)
                                    .set_media_type(media_type)
                                    .set_display(DisplaySuggestion {
                                        binary: Some(BinaryDisplay::Viewer),
                                        ..Default::default()
                                    }),
                            )
                        })
                        .collect(),
                );

                Ok(Output::new()
                    .set_title("Hex Decoder")
                    .add_blocks_iter(blocks)
                    .add_metadata(docs))
            }
            Mode::Encode => Ok(Output::new()
                .set_title("Hex Encoder")
                .add_blocks_iter(
                    args.data
                        .into_iter()
                        .chain(streams)
                        .map(|data| block::Data::new().set_text_data(hex::encode(data))),
                )
                .add_metadata(docs)),
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
