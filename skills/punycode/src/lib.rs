#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::{ArgEnum, Parser};
use mediatype::media_type;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"9Kr7qeDGzvzR8ph-ZyuQm",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_translator(PunycodeTranslator)
            .set_command(PunycodeCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct PunycodeTranslator;

impl Translator for PunycodeTranslator {
    fn parse(&self, args: &[Token]) -> Option<CommandArgs> {
        let decode = !args.is_empty()
            && args.iter().all(|token| {
                token.as_utf8().is_ascii()
                    && token
                        .as_utf8()
                        .to_ascii_lowercase()
                        .split('.')
                        .filter(|part| {
                            part.starts_with("xn--")
                                && punycode::decode(part.trim_start_matches("xn--")).is_ok()
                        })
                        .count()
                        > 0
            });

        if decode {
            return Some(
                CommandArgs::new()
                    .add_args(["--mode", "decode"])
                    .add_args_iter(args.iter().map(|token| token.as_utf8())),
            );
        }

        None
    }
}

#[derive(Debug)]
pub struct PunycodeCommand;

impl Command for PunycodeCommand {
    fn run(&self, args: CommandArgs) -> Result<Output, Output> {
        let args = Args::try_parse_from(args.args).unwrap();

        let output = args
            .inputs
            .into_iter()
            .map(|s| {
                let s = s
                    .to_ascii_lowercase()
                    .split('.')
                    .map(|section| {
                        if let Ok(decoded) = punycode::decode(section.trim_start_matches("xn--")) {
                            decoded
                        } else {
                            section.to_string()
                        }
                    })
                    .collect::<Vec<_>>();
                s.join(".")
            })
            .collect::<Vec<_>>();

        Ok(Output {
            module: "Punycode Decoder".into(),
            sections: vec![
                Section::new("Decoding punycode".to_string(), media_type!(TEXT / PLAIN))
                    .kind(SectionKind::Comment),
                Section::new(output.join("\n"), media_type!(TEXT / PLAIN)),
            ],
        })
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,

    #[clap(arg_enum, short, long)]
    pub mode: Mode,
}

#[derive(ArgEnum, Clone)]
enum Mode {
    Decode,
}
