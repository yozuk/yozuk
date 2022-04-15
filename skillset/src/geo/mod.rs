#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use mediatype::MediaType;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"nCK0EVzqv9V0ZTStJRjQn",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_preprocessor(GeoPreprocessor)
            .add_translator(GeoTranslator)
            .set_command(GeoCommand)
            .build()
    },
};

#[derive(Debug)]
struct GeoPreprocessor;

impl Preprocessor for GeoPreprocessor {
    fn preprocess(&self, input: Vec<Token>) -> Vec<Token> {
        input
            .into_iter()
            .map(|token| {
                let tag = if open_location_code::is_full(token.as_utf8()) {
                    "geo:olc".into()
                } else {
                    token.tag
                };
                Token { tag, ..token }
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct GeoTranslator;

impl Translator for GeoTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let codes = args
            .iter()
            .filter(|arg| arg.tag == "geo:olc")
            .collect::<Vec<_>>();

        if let [code] = codes[..] {
            return Some(CommandArgs::new().add_args(["--olc", code.as_utf8()]));
        }

        None
    }
}

#[derive(Debug)]
pub struct GeoCommand;

impl Command for GeoCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _locale: &Locale,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let code = open_location_code::decode(&args.olc.unwrap()).unwrap();
        Ok(Output {
            module: "Geo".into(),
            sections: vec![Section::new(
                format!("{} {}", code.center.x(), code.center.y()),
                MediaType::parse("application/vnd.yozuk.geo+json").unwrap(),
            )],
        })
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub olc: Option<String>,
}
