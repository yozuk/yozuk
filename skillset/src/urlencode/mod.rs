#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"6uuGAB41Wm0UUKduj9xtA",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_translator(UrlEncodeTranslator)
            .set_command(UrlEncodeCommand)
            .build()
    },
};

fn is_urlencoded(s: &str) -> bool {
    if let Ok(data) = urlencoding::decode(s) {
        data != s
    } else {
        false
    }
}

#[derive(Debug)]
pub struct UrlEncodeTranslator;

impl Translator for UrlEncodeTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let is_urlencoded = args.iter().all(|arg| is_urlencoded(arg.as_str()));
        if is_urlencoded {
            return Some(CommandArgs::new().add_args_iter(args.iter().map(|arg| arg.as_str())));
        }
        None
    }
}

#[derive(Debug)]
pub struct UrlEncodeCommand;

impl Command for UrlEncodeCommand {
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
            .filter_map(|arg| urlencoding::decode(arg).ok())
            .map(|data| block::Data::new().set_text_data(data));
        Ok(Output::new()
            .set_title("URL Decoder")
            .add_block(block::Comment::new().set_text("Decoding URL encoding"))
            .add_blocks(blocks))
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
