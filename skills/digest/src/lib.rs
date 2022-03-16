#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use mediatype::media_type;
use std::collections::BTreeMap;
use std::io::Read;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"nGU_LGgl3rNUVgwjlZVYl",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(DigestCorpus)
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
        ENTRIES
            .iter()
            .flat_map(|entry| entry.keywords)
            .flat_map(|key| {
                [
                    tk!([key.to_string(); "digest:keyword"]),
                    tk!([key.to_string(); "digest:keyword", key.to_string(); "digest:keyword"]),
                ]
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct DigestTranslator;

impl Translator for DigestTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "digest:keyword")
            .collect::<Vec<_>>();

        if !keywords.is_empty()
            && keywords.iter().all(|arg| {
                ENTRIES
                    .iter()
                    .any(|entry| normalized_eq(arg.as_utf8(), entry.keywords, 0))
            })
        {
            return Some(
                CommandArgs::new().add_args_iter(
                    keywords
                        .iter()
                        .flat_map(|arg| ["--algorithm", arg.as_utf8()]),
                ),
            );
        }

        None
    }
}

#[derive(Debug)]
pub struct DigestCommand;

impl Command for DigestCommand {
    fn run(&self, args: CommandArgs, streams: &mut [InputStream]) -> Result<Output, Output> {
        let args = Args::try_parse_from(args.args).unwrap();

        let mut entries = BTreeMap::new();
        for name in &args.algorithm {
            let matched = ENTRIES
                .iter()
                .filter(|entry| normalized_eq(name, entry.keywords, 0))
                .collect::<Vec<_>>();

            if matched.is_empty() {
                return Err(Output {
                    module: "Digest".into(),
                    sections: vec![Section::new(
                        format!("Unsupprted algorithm: {}", name),
                        media_type!(TEXT / PLAIN),
                    )
                    .kind(SectionKind::Comment)],
                });
            }

            for entry in matched {
                entries.entry(entry.name).or_insert_with(|| (entry.init)());
            }
        }

        if let [stream, ..] = streams {
            let mut data = vec![0; 1024];
            while let Ok(len) = stream.read(&mut data) {
                if len > 0 {
                    for hash in entries.values_mut() {
                        hash.update(&data[..len]);
                    }
                } else {
                    let result = entries
                        .iter_mut()
                        .map(|(name, hash)| format!("{}: {}", name, hex::encode(hash.finalize())))
                        .collect::<Vec<_>>();
                    return Ok(Output {
                        module: "Digest".into(),
                        sections: vec![Section::new(result.join("\n"), media_type!(TEXT / PLAIN))
                            .kind(SectionKind::Value)],
                    });
                }
            }
        }

        Err(Output {
            module: "Digest".into(),
            sections: vec![Section::new(
                "Invalid input source".to_string(),
                media_type!(TEXT / PLAIN),
            )
            .kind(SectionKind::Comment)],
        })
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, multiple_occurrences(true))]
    pub algorithm: Vec<String>,
}
