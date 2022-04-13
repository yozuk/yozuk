#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use mediatype::MediaType;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"ascHfHdlzryVq7Rvlpzsn",
    config_schema: None,
    init: |env, _| {
        Skill::builder()
            .add_corpus(VersionCorpus)
            .add_translator(VersionTranslator)
            .set_command(VersionCommand(env.clone()))
            .build()
    },
};

#[derive(Debug)]
pub struct VersionCorpus;

impl Corpus for VersionCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!([
                "build"; "version:keyword",
                "info"; "version:keyword"
            ]),
            tk!([
                "show",
                "build"; "version:keyword",
                "info"; "version:keyword"
            ]),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Debug)]
pub struct VersionTranslator;

impl Translator for VersionTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "version:keyword")
            .collect::<Vec<_>>();

        if let [build, info] = keywords[..] {
            if normalized_eq(build.as_utf8(), &["build"], 1)
                && normalized_eq(info.as_utf8(), &["info"], 1)
            {
                return Some(CommandArgs::new().add_args(["--build-info"]));
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct VersionCommand(Environment);

impl Command for VersionCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _locale: &Locale,
    ) -> Result<Output, CommandError> {
        let _args = Args::try_parse_from(args.args)?;
        Ok(Output {
            module: "Version Info".into(),
            sections: vec![Section::new(
                self.0.build_info.to_string(),
                MediaType::parse("application/vnd.yozuk.version+json").unwrap(),
            )],
        })
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub build_info: bool,
}
