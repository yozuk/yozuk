#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"OfHH5YRaI_kKKk7e56tCZ",
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
                "version"; "version:keyword",
                "info"; "version:keyword"
            ]),
            tk!([
                "show",
                "version"; "version:keyword",
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
            if normalized_eq(build.as_str(), &["version"], 1)
                && normalized_eq(info.as_str(), &["info"], 1)
            {
                return Some(CommandArgs::new().add_args(["--version-info"]));
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
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let _args = Args::try_parse_from(args.args)?;
        Ok(Output::new().set_title("Version Info").add_block(
            block::Data::new()
                .set_data(self.0.build_info.to_string())
                .set_media_type(media_type!(APPLICATION / JSON)),
        ))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub version_info: bool,
}
