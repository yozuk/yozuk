#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use lipsum::lipsum;
use lipsum::MarkovChain;
use serde_derive::Deserialize;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"WKQjgQTbySg0_NOxuUHBD",
    config_schema: Some(include_str!("./schema.json")),
    init: |_, config| {
        Skill::builder()
            .add_corpus(LipsumCorpus)
            .add_translator(LipsumTranslator)
            .set_command(LipsumCommand(config.get()))
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
                "Lorem"; "lipsum:keyword",
                "ipsum"; "lipsum:keyword",
                "dolor",
                "sit",
                "amet",
                "100"; "input:count"
            ]),
            tk!([
                "Lorem"; "lipsum:keyword",
                "ipsum"; "lipsum:keyword",
                "dolor",
                "sit",
                "amet",
                "100"; "input:count",
                "words"
            ]),
            tk!([
                "lorem"; "lipsum:keyword",
                "ipsum"; "lipsum:keyword"
            ]),
            tk!([
                "lorem"; "lipsum:keyword",
                "ipsum"; "lipsum:keyword",
                "100"; "input:count"
            ]),
            tk!([
                "lorem"; "lipsum:keyword",
                "ipsum"; "lipsum:keyword",
                "100"; "input:count",
                "words"
            ]),
            tk!(["lipsum,"; "lipsum:keyword"]),
            tk!([
                "lipsum,"; "lipsum:keyword",
                "100"; "input:count"
            ]),
            tk!([
                "lipsum,"; "lipsum:keyword",
                "100"; "input:count",
                "words"
            ]),
            tk!([
                "dummy"; "lipsum:keyword",
                "text"; "lipsum:keyword"
            ]),
            tk!([
                "Generate",
                "dummy"; "lipsum:keyword",
                "text"; "lipsum:keyword"
            ]),
            tk!([
                "dummy"; "lipsum:keyword",
                "text"; "lipsum:keyword",
                "100"; "input:count"
            ]),
            tk!([
                "dummy"; "lipsum:keyword",
                "text"; "lipsum:keyword",
                "100"; "input:count",
                "words"
            ]),
            tk!([
                "Generate",
                "100"; "input:count",
                "words",
                "dummy"; "lipsum:keyword",
                "text"; "lipsum:keyword"
            ]),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Debug)]
pub struct LipsumTranslator;

impl Translator for LipsumTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let count = args
            .iter()
            .find(|arg| arg.tag == "input:count")
            .and_then(|arg| arg.as_str().parse::<usize>().ok())
            .map(|n| ["-n".to_string(), n.to_string()]);

        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "lipsum:keyword")
            .collect::<Vec<_>>();

        if let [lorem, ipsum] = keywords[..] {
            if normalized_eq(lorem.as_str(), &["lorem"], 1)
                && normalized_eq(ipsum.as_str(), &["ipsum"], 1)
            {
                return Some(CommandArgs::new().add_args(count));
            }
        }

        if let [dummy, text] = keywords[..] {
            if normalized_eq(dummy.as_str(), &["dummy"], 1)
                && normalized_eq(text.as_str(), &["text"], 1)
            {
                return Some(CommandArgs::new().add_args(count));
            }
        }

        if let [lipsum] = keywords[..] {
            if normalized_eq(lipsum.as_str(), &["lipsum"], 1) {
                return Some(CommandArgs::new().add_args(count));
            }
        }

        None
    }
}

const MAX_COUNT: usize = 300;

#[derive(Debug)]
pub struct LipsumCommand(LipsumConfig);

impl Command for LipsumCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let chain = self.0.custom_text.as_ref().map(|text| {
            let mut chain = MarkovChain::new();
            chain.learn(text);
            chain
        });

        let args = Args::try_parse_from(args.args)?;
        if args.n > MAX_COUNT {
            return Err(Output {
                title: "Lorem ipsum".into(),
                blocks: vec![Block::Comment(block::Comment::new().set_text(format!(
                    "Too large number of the requested words (Limit: {}).",
                    MAX_COUNT
                )))],
                ..Default::default()
            }
            .into());
        }
        Ok(Output {
            blocks: vec![Block::Data(block::Data::new().set_text_data(
                if let Some(chain) = chain {
                    chain.generate(args.n)
                } else {
                    lipsum(args.n)
                },
            ))],
            ..Default::default()
        })
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, default_value_t = 30)]
    pub n: usize,
}

#[derive(Debug, Default, Clone, Deserialize)]
struct LipsumConfig {
    #[serde(default)]
    custom_text: Option<String>,
}
