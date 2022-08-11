use clap::{ArgEnum, Parser};
use itertools::iproduct;
use std::io::Read;
use yozuk_helper_encoding::EncodingPreprocessor;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::encoding::RawEncoding;
use yozuk_sdk::prelude::*;
use yozuk_sdk::Bytes;

mod algorithm;
use algorithm::ENTRIES;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"2s6Mbyyd74_slAdw_DHdN",
    init: |_| {
        Skill::builder()
            .add_corpus(CompressionCorpus)
            .add_preprocessor(EncodingPreprocessor::new(RawEncoding::all()))
            .add_suggestions(CompressionSuggestions)
            .add_translator(CompressionTranslator)
            .set_command(CompressionCommand)
            .build()
    },
};

pub struct CompressionSuggestions;

impl Suggestions for CompressionSuggestions {
    fn suggestions(&self, _seed: u64, args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        let inputs = args
            .iter()
            .filter(|arg| arg.tag == "input:data")
            .map(|arg| arg.as_str())
            .collect::<Vec<_>>();
        let joined = shell_words::join(if inputs.is_empty() {
            vec!["Hello World!"]
        } else {
            inputs
        });
        ENTRIES
            .iter()
            .filter_map(|entry| entry.keywords.iter().next())
            .map(|s| format!("{joined} to {s}"))
            .collect()
    }
}

pub struct CompressionCorpus;

impl Corpus for CompressionCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        let inputs = vec![
            "Hello World",
            "😍😗😋",
            "quick brown fox jumps over the lazy dog",
            "Veterinarian",
        ];
        iproduct!(
            inputs.clone(),
            ["as", "to", "in", "into"],
            ENTRIES.iter().flat_map(|entry| entry.keywords)
        )
        .map(|(data, prefix, alg)| {
            tk!([
                data; "input:data",
                prefix,
                *alg; "input:alg"
            ])
        })
        .collect()
    }
}

pub struct CompressionTranslator;

impl Translator for CompressionTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let input = args
            .iter()
            .filter(|arg| arg.tag == "input:data")
            .flat_map(|arg| ["--input", arg.as_str()]);

        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "input:alg")
            .collect::<Vec<_>>();

        if !keywords.is_empty()
            && keywords.iter().all(|arg| {
                ENTRIES
                    .iter()
                    .any(|entry| normalized_eq(arg.as_str(), entry.keywords, 0))
            })
        {
            return Some(
                CommandArgs::new().add_args_iter(input).add_args_iter(
                    keywords
                        .iter()
                        .flat_map(|arg| ["--algorithm", arg.as_str()]),
                ),
            );
        }

        None
    }
}

pub struct CompressionCommand;

impl Command for CompressionCommand {
    fn run(
        &self,
        args: CommandArgs,
        streams: &mut [InputStream],
        _user: &UserContext,
    ) -> Result<Output, CommandError> {
        let streams = streams.iter_mut().map(|stream| {
            stream
                .bytes()
                .map(|b| b.unwrap_or_default())
                .collect::<Bytes>()
        });
        let options = Options::try_parse_from(args.args)?;
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/compression/")?;
        match options.mode {
            Mode::Decompression => {
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
                    .set_title("Decompressor")
                    .add_blocks_iter(blocks)
                    .add_metadata(docs))
            }
            Mode::Compression => Ok(Output::new()
                .set_title("Compressor")
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
    Decompression,
    Compression,
}
