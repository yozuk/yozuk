use clap::Parser;
use rand::Rng;
use std::collections::HashSet;
use yozuk_helper_english::normalize;
use yozuk_sdk::prelude::*;

mod script;
use script::scripts;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"zl9hD8szURy_8p4Q5l21U",
    init: |_| {
        Skill::builder()
            .add_corpus(SmalltalkCorpus)
            .add_translator(SmalltalkTranslator)
            .set_command(SmalltalkCommand)
            .build()
    },
};

pub struct Script {
    pub title: Option<&'static str>,
    pub tokens: Vec<Vec<Token>>,
    pub responses: &'static [&'static str],
}

#[derive(Debug)]
pub struct SmalltalkCorpus;

impl Corpus for SmalltalkCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        scripts()
            .values()
            .flat_map(|def| def.tokens.clone())
            .collect()
    }
}

#[derive(Debug)]
pub struct SmalltalkTranslator;

impl Translator for SmalltalkTranslator {
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
            .filter_map(|key| scripts().get(key.as_str()).map(|item| (key, item)))
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
pub struct SmalltalkCommand;

impl Command for SmalltalkCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        if let Some(item) = scripts().get(args.name.as_str()) {
            let mut csrng = rand::thread_rng();
            let res = item.responses[csrng.gen_range(0..item.responses.len())];
            Ok(Output::new()
                .set_title(item.title.unwrap_or("Yozuk"))
                .add_block(block::Comment::new().set_text(res)))
        } else {
            Ok(Output::new()
                .set_title("Yozuk")
                .add_block(block::Comment::new().set_text("Hi. I'm Yozuk.")))
        }
    }

    fn priority(&self) -> i32 {
        -50
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub name: String,
}
