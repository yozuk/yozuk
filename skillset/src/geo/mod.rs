#![forbid(unsafe_code)]
#![deny(clippy::all)]

use anyhow::anyhow;
use clap::Parser;
use serde_derive::Serialize;
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
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let code = open_location_code::decode(&args.olc.unwrap())
            .map_err(|_| anyhow!("failed to decode the open location code"))?;
        let code = CodeArea {
            south: code.south,
            west: code.west,
            north: code.north,
            east: code.east,
            center: (code.center.x(), code.center.y()),
        };
        Ok(Output {
            title: "Geo".into(),
            blocks: vec![
                Block::Comment(block::Comment::new().set_text("Decoding Open Location Code")),
                Block::Data(block::Data::new().set_yaml_data(&code)?),
            ],
            ..Default::default()
        })
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub olc: Option<String>,
}

#[derive(Serialize)]
struct CodeArea {
    south: f64,
    west: f64,
    north: f64,
    east: f64,
    center: (f64, f64),
}
