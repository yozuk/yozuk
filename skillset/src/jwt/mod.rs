#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use min_jwt::UnverifiedJwt;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"881SN07tdT529jbaAYLwX",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_translator(JwtTranslator)
            .set_command(JwtCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct JwtTranslator;

impl Translator for JwtTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let is_jwt = args
            .iter()
            .all(|arg| UnverifiedJwt::with_str(arg.as_str()).is_ok());
        if is_jwt {
            return Some(CommandArgs::new().add_args_iter(args.iter().map(|arg| arg.as_str())));
        }
        None
    }
}

#[derive(Debug)]
pub struct JwtCommand;

impl Command for JwtCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let blocks = args
            .inputs
            .iter()
            .filter_map(|arg| UnverifiedJwt::with_str(arg).ok())
            .filter_map(|jwt| {
                jwt.decode_header()
                    .ok()
                    .and_then(|header| String::from_utf8(header).ok())
            })
            .flat_map(|header| {
                vec![
                    Block::Comment(block::Comment::new().set_text("Decoding JWT")),
                    Block::Data(block::Data::new().set_text_data(header)),
                ]
            });
        Ok(Output::new().set_title("JWT Decoder").add_blocks(blocks))
    }

    fn priority(&self) -> i32 {
        -100
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,
}
