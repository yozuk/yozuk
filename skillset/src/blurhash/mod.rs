#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use yozuk_sdk::prelude::*;

mod base83;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"DAyoatvv8aj-BkVUo-18M",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_translator(BlurHashTranslator)
            .set_command(BlurHashCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct BlurHashTranslator;

impl Translator for BlurHashTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let is_blurhash = args.iter().all(|arg| base83::validate_blurhash(&arg.data));
        if is_blurhash {
            return Some(CommandArgs::new().add_args_iter(args.iter().map(|arg| arg.as_str())));
        }
        None
    }
}

#[derive(Debug)]
pub struct BlurHashCommand;

impl Command for BlurHashCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let blocks = args.inputs.iter().flat_map(|arg| {
            let pixels = blurhash::decode(arg.as_str(), 50, 50, 1.0);
            vec![
                Block::Comment(block::Comment::new().set_text("Decoding BlurHash")),
                Block::Data(block::Data::new().set_data(pixels)),
            ]
        });
        Ok(Output::new()
            .set_title("BlurHash Decoder")
            .add_blocks_iter(blocks))
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,
}
