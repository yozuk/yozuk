use clap::Parser;
use itertools::iproduct;
use std::collections::BTreeMap;
use std::io::{BufReader, Read};
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"ZHvxjqf7uAilPbetXFrU-",
    init: |_| {
        Skill::builder()
            .add_corpus(DigestCorpus)
            .add_suggests(DigestSuggests)
            .add_translator(DigestTranslator)
            .set_command(DigestCommand)
            .build()
    },
};

mod algorithm;
use algorithm::*;

#[derive(Debug)]
pub struct DigestCorpus;

impl Corpus for DigestCorpus {
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
                *alg; "keyword"
            ])
        })
        .chain(
            ENTRIES
                .iter()
                .flat_map(|entry| entry.keywords)
                .flat_map(|key| {
                    [
                        tk!([key.to_string(); "keyword"]),
                        tk!([key.to_string(); "keyword", key.to_string(); "keyword"]),
                    ]
                }),
        )
        .collect()
    }
}

#[derive(Debug)]
pub struct DigestSuggests;

impl Suggests for DigestSuggests {
    fn suggests(&self, _seed: u64, _args: &[Token], streams: &[InputStream]) -> Vec<String> {
        if streams.is_empty() {
            vec![]
        } else {
            ENTRIES
                .iter()
                .filter_map(|entry| entry.keywords.iter().next())
                .map(|s| s.to_string())
                .collect()
        }
    }
}

#[derive(Debug)]
pub struct DigestTranslator;

impl Translator for DigestTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let input = args
            .iter()
            .filter(|arg| arg.tag == "input:data")
            .flat_map(|arg| ["--input", arg.as_str()]);

        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "keyword")
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

#[derive(Debug)]
pub struct DigestCommand;

impl Command for DigestCommand {
    fn run(
        &self,
        args: CommandArgs,
        streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;

        let mut entries = BTreeMap::new();
        for name in &args.algorithm {
            let matched = ENTRIES
                .iter()
                .filter(|entry| normalized_eq(name, entry.keywords, 0))
                .collect::<Vec<_>>();

            if matched.is_empty() {
                return Err(Output::new()
                    .set_title("Digest")
                    .add_block(
                        block::Comment::new().set_text(format!("Unsupprted algorithm: {}", name)),
                    )
                    .into());
            }

            for entry in matched {
                entries.entry(entry.name).or_insert_with(|| (entry.init)());
            }
        }

        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/digest/")?;
        if let [input, ..] = &args.input[..] {
            let mut input = input.as_bytes();
            if let Some(output) = compute_hash(&mut input, entries) {
                return Ok(output.add_metadata(docs));
            }
        } else if let [stream, ..] = streams {
            let mut reader = BufReader::new(stream);
            if let Some(output) = compute_hash(&mut reader, entries) {
                return Ok(output.add_metadata(docs));
            }
        }

        Err(Output::new()
            .set_title("Digest")
            .add_block(block::Comment::new().set_text("No valid input source provided"))
            .add_metadata(docs)
            .into())
    }
}

fn compute_hash(
    reader: &mut dyn Read,
    mut entries: BTreeMap<&'static str, Box<dyn Algorithm>>,
) -> Option<Output> {
    let mut data = vec![0; 1024];
    while let Ok(len) = reader.read(&mut data) {
        if len > 0 {
            for hash in entries.values_mut() {
                hash.update(&data[..len]);
            }
        } else {
            let mut entries = entries.into_iter().collect::<Vec<_>>();
            let result = if entries.len() == 1 {
                vec![hex::encode(entries[0].1.finalize())]
            } else {
                entries
                    .into_iter()
                    .map(|(name, mut hash)| format!("{}: {}", name, hex::encode(hash.finalize())))
                    .collect::<Vec<_>>()
            };

            return Some(
                Output::new()
                    .set_title("Digest")
                    .add_block(block::Data::new().set_text_data(result.join("\n"))),
            );
        }
    }
    None
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, multiple_occurrences(true))]
    pub algorithm: Vec<String>,
    #[clap(short, long, multiple_occurrences(true))]
    pub input: Vec<String>,
}
