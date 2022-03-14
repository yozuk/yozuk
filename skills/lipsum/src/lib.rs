#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use lipsum::lipsum;
use mediatype::media_type;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"CI5cJBeuNmRj5mMRVAC_X",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(LipsumCorpus)
            .add_translator(LipsumTranslator)
            .set_command(LipsumCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct LipsumCorpus;

impl Corpus for LipsumCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!([
                "Lorem"; "lipsum:keyword",
                "ipsum"; "lipsum:keyword",
                "dolor",
                "sit",
                "amet"
            ]),
            tk!([
                "lorem"; "lipsum:keyword",
                "ipsum"; "lipsum:keyword"
            ]),
            tk!(["lipsum,"; "lipsum:keyword"]),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Debug)]
pub struct LipsumTranslator;

impl Translator for LipsumTranslator {
    fn parse(&self, args: &[Token]) -> Option<CommandArgs> {
        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "lipsum:keyword")
            .collect::<Vec<_>>();

        if let [lorem, ipsum] = keywords[..] {
            if normalized_eq(lorem.as_utf8(), &["lorem"], 1)
                && normalized_eq(ipsum.as_utf8(), &["ipsum"], 1)
            {
                return Some(CommandArgs::new());
            }
        }

        if let [lipsum] = keywords[..] {
            if normalized_eq(lipsum.as_utf8(), &["lipsum"], 1) {
                return Some(CommandArgs::new());
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct LipsumCommand;

impl Command for LipsumCommand {
    fn run(&self, args: CommandArgs) -> Result<Output, Output> {
        let args = Args::try_parse_from(args.args).unwrap();
        Ok(Output {
            sections: vec![
                Section::new(lipsum(args.n), media_type!(TEXT / PLAIN)).kind(SectionKind::Value)
            ],
            ..Default::default()
        })
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, default_value_t = 30)]
    pub n: usize,
}
