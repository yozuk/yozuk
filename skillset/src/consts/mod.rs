use clap::Parser;
use yozuk_sdk::prelude::*;
use std::collections::HashSet;

mod definition;
use definition::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"bQ49wkMRKLOjoZ17U0-i9",
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
    pub unit: Option<&'static str>,
    pub is_exact: bool,
}

#[derive(Debug)]
pub struct ConstCorpus;

impl Corpus for ConstCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        DEFINITIONS.values().flat_map(|def| def.tokens.clone()).collect()
    }
}

#[derive(Debug)]
pub struct ConstTranslator;

impl Translator for ConstTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let keys: HashSet<String> = args.iter()
        .map(|arg| arg.tag.clone())
        .filter(|tag| tag.starts_with("keyword:"))
        .map(|tag| tag.trim_start_matches("keyword:").to_string())
        .collect();
        None
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
        let _args = Args::try_parse_from(args.args)?;
        Ok(Output::new().add_block(block::Comment::new().set_text("Hi. I'm Yozuk.")))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub name: String,
}
