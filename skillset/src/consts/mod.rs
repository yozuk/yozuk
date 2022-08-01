use clap::Parser;
use std::collections::HashSet;
use thousands::Separable;
use yozuk_helper_english::normalize;
use yozuk_sdk::prelude::*;

mod definition;
use definition::DEFINITIONS;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"sAMlRNhLB1Ww9T21BEDMl",
    init: |_| {
        Skill::builder()
            .add_corpus(ConstCorpus)
            .add_translator(ConstTranslator)
            .add_suggestions(ConstSuggestions)
            .set_command(ConstCommand)
            .build()
    },
};

pub struct ConstSuggestions;

impl Suggestions for ConstSuggestions {
    fn suggestions(&self, _seed: u64, _args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        vec!["How fast the speed of light?"]
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

pub struct Constant {
    pub name: &'static str,
    pub tokens: fn() -> Vec<Vec<Token>>,
    pub value: &'static str,
    pub scale: i32,
    pub unit: Option<&'static str>,
    pub is_exact: bool,
}

impl Default for Constant {
    fn default() -> Self {
        Self {
            name: "",
            tokens: Vec::new,
            value: "",
            scale: 0,
            unit: None,
            is_exact: false,
        }
    }
}

pub struct ConstCorpus;

impl Corpus for ConstCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        DEFINITIONS
            .values()
            .flat_map(|def| (def.tokens)())
            .collect()
    }
}

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
            .filter_map(|key| DEFINITIONS.get(key.as_str()).map(|item| (key, item)))
            .find(|(_, item)| {
                (item.tokens)().iter().any(|tokens| {
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
                    .map(|unit| format!("{}{} {}", item.value.separate_with_commas(), scale, unit))
                    .unwrap_or_else(|| {
                        format!("{}{}", item.value.to_string().separate_with_commas(), scale)
                    });
                vec![Block::Data(block::Data::new().set_highlighted_text_data(
                    format!("{}\n`{}`", comment, value),
                    &Default::default(),
                ))]
            });
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/consts/")?;
        Ok(Output::new()
            .set_title("Constants")
            .add_blocks_iter(blocks)
            .add_metadata(docs))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub name: String,
}
