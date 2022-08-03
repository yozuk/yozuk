use anyhow::anyhow;
use clap::Parser;
use serde_derive::Serialize;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"nCK0EVzqv9V0ZTStJRjQn",
    init: |_| {
        Skill::builder()
            .add_preprocessor(GeoPreprocessor)
            .add_translator(GeoTranslator)
            .set_command(GeoCommand)
            .build()
    },
};

struct GeoPreprocessor;

impl Preprocessor for GeoPreprocessor {
    fn preprocess(&self, input: Vec<Token>) -> Vec<Token> {
        input
            .into_iter()
            .map(|token| {
                let tag = if open_location_code::is_full(token.as_str()) {
                    "geo:olc".into()
                } else {
                    token.tag
                };
                Token { tag, ..token }
            })
            .collect()
    }
}

pub struct GeoTranslator;

impl Translator for GeoTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let codes = args
            .iter()
            .filter(|arg| arg.tag == "geo:olc")
            .collect::<Vec<_>>();

        if let [code] = codes[..] {
            return Some(CommandArgs::new().add_args(["--olc", code.as_str()]));
        }

        None
    }
}

pub struct GeoCommand;

impl Command for GeoCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _user: &UserContext,
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

        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/geo/")?;
        Ok(Output::new()
            .set_title("Geo")
            .add_blocks_iter(vec![
                Block::Comment(block::Comment::new().set_text("Decoding Open Location Code")),
                Block::Data(block::Data::new().set_json_data(&code)?),
            ])
            .add_metadata(docs)
            .add_metadata(Metadata::link(
                "Geo URI",
                format!("geo:{},{}", code.center.0, code.center.1),
            )?)
            .add_metadata(Metadata::link(
                "OpenStreetMap",
                format!(
                    "https://www.openstreetmap.org/#map=10/{}/{}",
                    code.center.0, code.center.1
                ),
            )?))
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
