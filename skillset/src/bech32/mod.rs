use bech32::Variant;
use clap::Parser;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"6uuGAB41Wm0UUKduj9xtA",
    init: |_| {
        Skill::builder()
            .add_translator(Bech32Translator)
            .add_suggests(Bech32Suggests)
            .set_command(Bech32Command)
            .build()
    },
};

#[derive(Debug)]
pub struct Bech32Suggests;

impl Suggests for Bech32Suggests {
    fn random_suggests(&self) -> Vec<String> {
        vec![
            "b321fpjkcmr0yptk7unvvsss8ctgvx",
            "b32m1fpjkcmr0yptk7unvvssserxuym",
        ]
        .into_iter()
        .map(Into::into)
        .collect()
    }
}

#[derive(Debug)]
pub struct Bech32Translator;

impl Translator for Bech32Translator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let is_bech32 =
            !args.is_empty() && args.iter().all(|arg| bech32::decode(arg.as_str()).is_ok());
        if is_bech32 {
            return Some(CommandArgs::new().add_args_iter(args.iter().map(|arg| arg.as_str())));
        }
        None
    }
}

#[derive(Debug)]
pub struct Bech32Command;

impl Command for Bech32Command {
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
            .filter_map(|arg| bech32::decode(arg).ok())
            .flat_map(|(hrp, data, variant)| {
                let data = bech32::convert_bits(&data, 5, 8, true).unwrap();
                let variant = match variant {
                    Variant::Bech32 => "Bech32",
                    Variant::Bech32m => "Bech32m",
                };
                vec![
                    Block::Comment(block::Comment::new().set_text(format!("Decoding {}", variant))),
                    Block::Data(block::Data::new().set_text_data(hrp)),
                    Block::Data(block::Data::new().set_data(data)),
                ]
            });
        Ok(Output::new()
            .set_title("Bech32 Decoder")
            .add_blocks_iter(blocks))
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
