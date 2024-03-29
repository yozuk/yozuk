use bigdecimal::BigDecimal;
use bigdecimal::Signed;
use clap::Parser;
use std::str::FromStr;
use std::u64;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"nSz49UpiDtQLWUfAUZEq1",
    init: |_| {
        Skill::builder()
            .add_labeler(NumericLabeler)
            .add_translator(NumericTranslator)
            .set_command(NumericCommand)
            .build()
    },
};

fn label_numeric(token: &Token) -> impl Iterator<Item = Feature> {
    let mut features = Vec::new();
    if let Ok(n) = BigDecimal::from_str(token.as_str()) {
        features.push(Feature {
            name: "numeric".into(),
            ..Default::default()
        });
        features.push(if n.is_positive() {
            Feature {
                name: "numeric:positive".into(),
                ..Default::default()
            }
        } else if n.is_negative() {
            Feature {
                name: "numeric:negative".into(),
                ..Default::default()
            }
        } else {
            Feature {
                name: "numeric:zero".into(),
                ..Default::default()
            }
        });
        features.push(if n.is_integer() {
            Feature {
                name: "numeric:integer".into(),
                ..Default::default()
            }
        } else {
            Feature {
                name: "numeric:float".into(),
                ..Default::default()
            }
        });
    }
    features.into_iter()
}

pub struct NumericLabeler;

impl Labeler for NumericLabeler {
    fn label_features(&self, input: &[Token]) -> Vec<Vec<Feature>> {
        input
            .iter()
            .map(|token| label_numeric(token).collect())
            .collect()
    }
}

pub struct NumericTranslator;

impl Translator for NumericTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let is_integer =
            !args.is_empty() && args.iter().all(|arg| parse_int(arg.as_str()).is_some());
        if is_integer {
            if let [arg] = args {
                return Some(CommandArgs::new().add_args([arg.as_str()]));
            }
        }
        None
    }
}

pub struct NumericCommand;

impl Command for NumericCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _user: &UserContext,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let blocks = args
            .inputs
            .iter()
            .filter_map(|input| parse_int(input))
            .map(|(radix, num)| {
                let original = radix.format(num);
                let redixes = Radix::iter()
                    .filter(|&r| r != radix)
                    .map(|radix| format!("`{}`", radix.format(num)))
                    .collect::<Vec<_>>();

                block::Data::new().set_highlighted_text_data(
                    format!("{} =\n{}", original, redixes.join("\n")),
                    &Default::default(),
                )
            })
            .collect::<Vec<_>>();
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/numeric/")?;
        Ok(Output::new()
            .set_title("Base Converter")
            .add_blocks_iter(blocks)
            .set_mode(OutputMode::Attachment)
            .add_metadata(docs))
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

fn parse_int(s: &str) -> Option<(Radix, u64)> {
    Radix::iter().find_map(|radix| {
        if s.starts_with(radix.prefix()) {
            u64::from_str_radix(s.trim_start_matches(radix.prefix()), radix as _)
                .ok()
                .map(|num| (radix, num))
        } else {
            None
        }
    })
}

#[derive(EnumIter, Clone, Copy, PartialEq)]
enum Radix {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
}

impl Radix {
    fn prefix(&self) -> &'static str {
        match self {
            Self::Binary => "0b",
            Self::Octal => "0o",
            Self::Decimal => "",
            Self::Hexadecimal => "0x",
        }
    }

    fn format(&self, n: u64) -> String {
        match self {
            Self::Binary => format!("0b{:b}", n),
            Self::Octal => format!("0o{:o}", n),
            Self::Decimal => format!("{}", n),
            Self::Hexadecimal => format!("0x{:x}", n),
        }
    }
}
