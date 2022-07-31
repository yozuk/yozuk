use clap::Parser;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"OfHH5YRaI_kKKk7e56tCZ",
    init: |env| {
        Skill::builder()
            .add_corpus(VersionCorpus)
            .add_translator(VersionTranslator)
            .set_command(VersionCommand(env.clone()))
            .build()
    },
};

pub struct VersionCorpus;

impl Corpus for VersionCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!([
                "version"; "keyword",
                "info"; "keyword"
            ]),
            tk!([
                "show",
                "version"; "keyword",
                "info"; "keyword"
            ]),
        ]
        .into_iter()
        .collect()
    }
}

pub struct VersionTranslator;

impl Translator for VersionTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "keyword")
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

pub struct VersionCommand(Environment);

impl Command for VersionCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let _args = Args::try_parse_from(args.args)?;
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/version/")?;
        Ok(Output::new()
            .set_title("Version Info")
            .add_block(
                block::Data::new()
                    .set_data(self.0.build_info.to_string())
                    .set_media_type(media_type!(APPLICATION / JSON)),
            )
            .add_metadata(docs))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub version_info: bool,
}
