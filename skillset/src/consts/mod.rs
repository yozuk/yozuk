use clap::Parser;
use std::collections::HashSet;
use yozuk_helper_english::normalize;
use yozuk_sdk::prelude::*;

mod definition;
use definition::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"NA7YShJlZRtndZH5FqXc2",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(ConstCorpus)
            .add_translator(ConstTranslator)
            .set_command(ConstCommand)
            .build()
    },
};

#[derive(Default)]
pub struct Constant {
    pub name: &'static str,
    pub tokens: Vec<Vec<Token>>,
    pub value: &'static str,
    pub scale: i32,
    pub unit: Option<&'static str>,
    pub is_exact: bool,
}

#[derive(Debug)]
pub struct ConstCorpus;

impl Corpus for ConstCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        DEFINITIONS
            .values()
            .flat_map(|def| def.tokens.clone())
            .collect()
    }
}

#[derive(Debug)]
pub struct ConstTranslator;

impl Translator for ConstTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let keywords = args
            .iter()
            .filter(|arg| arg.tag.starts_with("keyword:"))
            .map(|arg| normalize(arg.as_str()))
            .collect::<HashSet<_>>();
        let keys: HashSet<String> = args
            .iter()
            .map(|arg| arg.tag.clone())
            .filter(|tag| tag.starts_with("keyword:"))
            .map(|tag| tag.trim_start_matches("keyword:").to_string())
            .collect();
        keys.into_iter()
            .filter_map(|key| DEFINITIONS.get(key.as_str()).map(|item| (key, item)))
            .find(|(_, item)| {
                item.tokens.iter().any(|tokens| {
                    tokens
                        .iter()
                        .filter(|arg| arg.tag.starts_with("keyword:"))
                        .map(|arg| normalize(arg.as_str()))
                        .all(|key| keywords.contains(&key))
                })
            })
            .map(|(key, _)| CommandArgs::new().add_args(["--name".to_string(), key]))
    }
}

#[derive(Debug)]
pub struct ConstCommand;

impl Command for ConstCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let blocks = DEFINITIONS
            .get(args.name.as_str())
            .into_iter()
            .flat_map(|item| {
                let comment = if item.is_exact {
                    format!("{} {}", item.name, "=")
                } else {
                    format!("{} {}", item.name, "≈")
                };
                let scale = if item.scale != 0 {
                    format!("e{}", item.scale)
                } else {
                    String::new()
                };
                let value = item
                    .unit
                    .map(|unit| format!("{}{} {}", item.value, scale, unit))
                    .unwrap_or_else(|| item.value.to_string());
                vec![
                    Block::Comment(block::Comment::new().set_text(comment)),
                    Block::Data(block::Data::new().set_text_data(value)),
                ]
            });
        Ok(Output::new().add_blocks_iter(blocks))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub name: String,
}
