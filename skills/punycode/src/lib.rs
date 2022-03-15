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
        let decode = !args.is_empty() && args.iter().all(|token| is_punycode(token.as_utf8()));

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
            .iter()
            .map(|s| encode_to_punycode(s))
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

fn is_punycode(s: &str) -> bool {
    s.is_ascii()
        && s.to_ascii_lowercase()
            .split('.')
            .filter(|part| {
                part.starts_with("xn--")
                    && punycode::decode(part.trim_start_matches("xn--")).is_ok()
            })
            .count()
            > 0
}

fn encode_to_punycode(s: &str) -> String {
    let s = s
        .to_ascii_lowercase()
        .split('.')
        .map(|part| {
            if part.starts_with("xn--") {
                punycode::decode(part.trim_start_matches("xn--"))
                    .ok()
                    .unwrap_or_default()
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>();
    s.join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_punycode() {
        assert!(is_punycode("xn--cookie!-1d84f"));
        assert!(is_punycode("xn--mushroom-bd25gia"));
        assert!(is_punycode("xn--li8h.and.xn--ri8h"));
        assert!(is_punycode("XN--COOKIE!-1D84F"));
        assert!(!is_punycode("example.com"));
    }

    #[test]
    fn test_encode_to_punycode() {
        assert_eq!(encode_to_punycode("xn--cookie!-1d84f"), "cookie🍪!");
        assert_eq!(encode_to_punycode("XN--COOKIE!-1D84F"), "cookie🍪!");
        assert_eq!(encode_to_punycode("xn--mushroom-bd25gia"), "🍄mushroom🍄");
        assert_eq!(encode_to_punycode("xn--li8h.and.xn--ri8h"), "🍋.and.🍑");
        assert_eq!(encode_to_punycode("example.com"), "example.com");
    }
}
