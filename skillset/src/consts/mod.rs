use clap::Parser;
use std::collections::HashSet;
use thousands::Separable;
use yozuk_helper_english::normalize;
use yozuk_sdk::prelude::*;

mod definition;
use definition::definitions;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"c0FBNukgxcKvGJ9stDZ8K",
    init: |_| {
        Skill::builder()
            .add_corpus(ConstCorpus)
            .add_translator(ConstTranslator)
            .add_suggestions(ConstSuggestions)
            .set_command(ConstCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct ConstSuggestions;

impl Suggestions for ConstSuggestions {
    fn suggestions(&self, _seed: u64, _args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        vec!["How fast the speed of light?"]
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

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
        definitions()
            .values()
            .flat_map(|def| def.tokens.clone())
            .collect()
    }
}

#[derive(Debug)]
pub struct ConstTranslator;

impl Translator for ConstTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
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
            .filter_map(|key| definitions().get(key.as_str()).map(|item| (key, item)))
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
        let blocks = definitions()
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
                    .map(|unit| format!("{}{} {}", item.value.separate_with_commas(), scale, unit))
                    .unwrap_or_else(|| item.value.to_string().separate_with_commas());
                vec![Block::Data(block::Data::new().set_highlighted_text_data(
                    format!("{}\n`{}`", comment, value),
                    &Default::default(),
                ))]
            });
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/consts/")?;
        Ok(Output::new().add_blocks_iter(blocks).add_metadata(docs))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub name: String,
}
