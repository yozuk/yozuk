use clap::{ArgEnum, Parser};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use yozuk_sdk::prelude::*;

mod tld;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"9Kr7qeDGzvzR8ph-ZyuQm",
    init: |_| {
        Skill::builder()
            .add_suggestions(PunycodeSuggestions)
            .add_translator(PunycodeTranslator)
            .set_command(PunycodeCommand)
            .build()
    },
};

pub struct PunycodeSuggestions;

impl Suggestions for PunycodeSuggestions {
    fn suggestions(&self, seed: u64, _args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        let mut rng = StdRng::seed_from_u64(seed);
        let emoji = ["🦊", "🐼", "🐰", "🐶", "🐯"].choose(&mut rng).unwrap();
        vec![format!("{emoji}.example.com")]
    }
}

pub struct PunycodeTranslator;

impl Translator for PunycodeTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let decode = !args.is_empty() && args.iter().all(|token| is_punycode(token.as_str()));

        if decode {
            return Some(
                CommandArgs::new()
                    .add_args(["--mode", "decode"])
                    .add_args_iter(args.iter().map(|token| token.as_str())),
            );
        }

        let encode =
            !args.is_empty() && args.iter().all(|token| is_non_ascii_domain(token.as_str()));

        if encode {
            return Some(
                CommandArgs::new()
                    .add_args(["--mode", "encode"])
                    .add_args_iter(args.iter().map(|token| token.as_str())),
            );
        }

        None
    }
}

pub struct PunycodeCommand;

impl Command for PunycodeCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _user: &UserContext,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;

        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/punycode/")?;
        match args.mode {
            Mode::Decode => {
                let output = args
                    .inputs
                    .iter()
                    .map(|s| decode_punycode(s))
                    .collect::<Vec<_>>();
                Ok(Output::new()
                    .set_title("Punycode Decoder")
                    .add_blocks_iter(vec![Block::Data(
                        block::Data::new().set_text_data(output.join("\n")),
                    )])
                    .add_metadata(docs))
            }
            Mode::Encode => {
                let output = args
                    .inputs
                    .iter()
                    .map(|s| encode_punycode(s))
                    .collect::<Vec<_>>();

                Ok(Output::new()
                    .set_title("Punycode Decoder")
                    .add_blocks_iter(vec![
                        Block::Comment(block::Comment::new().set_text("Encoding punycode")),
                        Block::Data(block::Data::new().set_text_data(output.join("\n"))),
                    ])
                    .add_metadata(docs))
            }
        }
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
    Encode,
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

fn decode_punycode(s: &str) -> String {
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

fn encode_punycode(s: &str) -> String {
    let s = s
        .to_ascii_lowercase()
        .split('.')
        .map(|part| {
            if part.is_ascii() {
                part.to_string()
            } else if let Ok(encoded) = punycode::encode(part) {
                format!("xn--{}", encoded)
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>();
    s.join(".")
}

fn is_non_ascii_domain(s: &str) -> bool {
    !s.is_ascii() && tld::DOMAINS.iter().any(|domain| s.ends_with(domain))
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
    fn test_is_non_ascii_domain() {
        assert!(is_non_ascii_domain("🍪.com"));
        assert!(is_non_ascii_domain("cookie.テスト"));
        assert!(!is_non_ascii_domain("xn--cookie!-1d84f.com"));
        assert!(!is_non_ascii_domain("🍋.yozuk"));
    }

    #[test]
    fn test_decode_punycode() {
        assert_eq!(decode_punycode("xn--cookie!-1d84f"), "cookie🍪!");
        assert_eq!(decode_punycode("XN--COOKIE!-1D84F"), "cookie🍪!");
        assert_eq!(decode_punycode("xn--mushroom-bd25gia"), "🍄mushroom🍄");
        assert_eq!(decode_punycode("xn--li8h.and.xn--ri8h"), "🍋.and.🍑");
        assert_eq!(decode_punycode("example.com"), "example.com");
    }

    #[test]
    fn test_encode_punycode() {
        assert_eq!(encode_punycode("cookie🍪!"), "xn--cookie!-1d84f");
        assert_eq!(encode_punycode("COOKIE🍪!"), "xn--cookie!-1d84f");
        assert_eq!(encode_punycode("🍄mushroom🍄"), "xn--mushroom-bd25gia");
        assert_eq!(encode_punycode("🍋.and.🍑"), "xn--li8h.and.xn--ri8h");
        assert_eq!(encode_punycode("example.com"), "example.com");
    }
}
