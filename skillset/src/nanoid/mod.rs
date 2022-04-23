use clap::Parser;
use itertools::iproduct;
use std::iter;
use yozuk_helper_english::{normalized_eq, pluralize};
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"RcVutnguDE51HEtdOHHC4",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(NanoIdCorpus)
            .add_suggests(NanoIdSuggests)
            .add_translator(NanoIdTranslator)
            .set_command(NanoIdCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct NanoIdCorpus;

impl Corpus for NanoIdCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        iproduct!(["generate", "new"], ["nanoid", "NanoID"])
            .flat_map(|(verb, name)| {
                vec![tk!([
                    verb,
                    name; "command:nanoid"
                ])]
            })
            .chain(
                iproduct!(["generate", "new"], ["nanoid", "NanoID"], 1..=10).flat_map(
                    |(verb, name, count)| {
                        vec![tk!([
                            verb,
                            format!("{}", count); "input:count",
                            name; "command:nanoid"
                        ])]
                    },
                ),
            )
            .chain(["nanoid", "NanoID"].map(|name| tk!([name; "command:nanoid"])))
            .collect()
    }
}

#[derive(Debug)]
pub struct NanoIdSuggests;

impl Suggests for NanoIdSuggests {
    fn suggests(&self, _input: &[Token]) -> Vec<String> {
        ["NanoID", "Genarate NanoID", "New NanoID"]
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

#[derive(Debug)]
pub struct NanoIdTranslator;

impl Translator for NanoIdTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        if !args
            .iter()
            .any(|arg| arg.tag == "command:nanoid" && normalized_eq(arg.as_utf8(), &["NanoID"], 0))
        {
            return None;
        }
        let count = args
            .iter()
            .find(|arg| arg.tag == "input:count")
            .and_then(|arg| arg.as_utf8().parse::<usize>().ok())
            .unwrap_or(1);
        Some(CommandArgs::new().add_args(["-n".to_string(), count.to_string()]))
    }
}

const MAX_COUNT: usize = 30;

#[derive(Debug)]
pub struct NanoIdCommand;

impl Command for NanoIdCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        if args.n > MAX_COUNT {
            return Err(Output {
                module: "NanoID Generator".into(),
                sections: vec![Section::new(
                    format!(
                        "Too large number of the requested NanoIDs (Limit: {}).",
                        MAX_COUNT
                    ),
                    media_type!(TEXT / PLAIN),
                )
                .kind(SectionKind::Comment)],
                blocks: vec![Block::Comment(block::Comment::new().set_text(format!(
                    "Too large number of the requested NanoIDs (Limit: {}).",
                    MAX_COUNT
                )))],
            }
            .into());
        }
        let list = iter::repeat_with(|| nanoid::nanoid!())
            .take(args.n)
            .collect::<Vec<_>>();
        Ok(Output {
            module: "NanoID Generator".into(),
            sections: vec![
                Section::new(
                    format!("Generating {} {}", args.n, pluralize("NanoID", args.n)),
                    media_type!(TEXT / PLAIN),
                )
                .kind(SectionKind::Comment),
                Section::new(list.join("\n"), media_type!(TEXT / PLAIN)),
            ],
            blocks: vec![
                Block::Comment(block::Comment::new().set_text(format!(
                    "Generating {} {}",
                    args.n,
                    pluralize("NanoID", args.n)
                ))),
                Block::Data(
                    block::Data::new()
                        .set_data(list.join("\n"))
                        .set_media_type(media_type!(TEXT / PLAIN)),
                ),
            ],
        })
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, default_value_t = 1)]
    pub n: usize,
}
