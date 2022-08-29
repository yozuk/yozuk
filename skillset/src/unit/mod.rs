use bigdecimal::{BigDecimal, One, ToPrimitive};
use clap::Parser;
use itertools::iproduct;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::str::FromStr;
use yozuk_helper_english::NumeralTokenParser;
use yozuk_sdk::prelude::*;
use yozuk_sdk::preprocessor::TokenMerger;

mod conversion;
mod entry;
mod symbol;
mod table;

use entry::*;
use table::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"3taUCmvXuvj6yElJl8GIk",
    init: |_| {
        Skill::builder()
            .add_preprocessor(TokenMerger::new(NumeralTokenParser))
            .add_preprocessor(UnitPreprocessor)
            .add_corpus(UnitCorpus)
            .add_translator(UnitTranslator)
            .add_suggestions(UnitSuggestions)
            .set_command(UnitCommand)
            .build()
    },
};

pub struct UnitSuggestions;

impl Suggestions for UnitSuggestions {
    fn suggestions(&self, seed: u64, _args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        let mut rng = StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(10..=1000);
        let unit = [
            ("km", "mi"),
            ("in", "mm"),
            ("hPa", "mmHg"),
            ("kg", "oz."),
            ("oz.", "mg"),
            ("KiB", "B"),
            ("mph", "km/h"),
            ("°F", "K"),
        ]
        .choose(&mut rng)
        .unwrap();
        vec![format!("{}{} to {}", n, unit.0, unit.1)]
    }
}

struct UnitPreprocessor;

impl Preprocessor for UnitPreprocessor {
    fn preprocess(&self, input: Vec<Token>) -> Vec<Token> {
        input
            .into_iter()
            .flat_map(|token| {
                if let Some((num, unit)) = symbol::parse_num_symbol(token.as_str()) {
                    vec![
                        Token {
                            data: num.to_string().into(),
                            ..token.clone()
                        },
                        Token {
                            data: unit.to_string().into(),
                            ..token
                        },
                    ]
                } else {
                    vec![token]
                }
            })
            .collect()
    }
}

pub struct UnitCorpus;

impl Corpus for UnitCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        ENTRIES
            .iter()
            .flat_map(|entry| {
                entry.symbols.iter().flat_map(|sym| {
                    entry
                        .prefixes
                        .iter()
                        .map(move |prefix| {
                            tk!([
                                "1.0"; "input:value",
                                format!("{}{}", prefix, sym); "keyword"
                            ])
                        })
                        .chain(Some(tk!([
                            "1.0"; "input:value",
                            sym.to_string(); "keyword"
                        ])))
                })
            })
            .chain(
                iproduct!(["as", "to", "in", "into"], ENTRIES).flat_map(|(prep, unit)| {
                    unit.symbols.iter().map(|sym| {
                        tk!([
                            "1.0"; "input:value",
                            format!("{}", sym); "keyword",
                            prep,
                            format!("{}", sym); "keyword"
                        ])
                    })
                }),
            )
            .collect()
    }
}

pub struct UnitTranslator;

impl Translator for UnitTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let values = args
            .iter()
            .filter(|arg| arg.tag == "input:value")
            .filter_map(|token| BigDecimal::from_str(token.as_str()).ok())
            .collect::<Vec<_>>();

        let units = args
            .iter()
            .filter(|arg| arg.tag == "keyword")
            .filter(|arg| symbol::parse_symbol(arg.as_str()).is_some())
            .collect::<Vec<_>>();

        if let [value] = &values[..] {
            if let [unit, to] = units[..] {
                return Some(CommandArgs::new().add_args([
                    "--value".to_string(),
                    format!(" {}", value),
                    "--unit".to_string(),
                    unit.as_str().to_string(),
                    "--to".to_string(),
                    to.as_str().to_string(),
                ]));
            } else if let [unit] = units[..] {
                return Some(CommandArgs::new().add_args([
                    "--value".to_string(),
                    format!(" {}", value),
                    "--unit".to_string(),
                    unit.as_str().to_string(),
                ]));
            }
        }

        None
    }
}

pub struct UnitCommand;

impl Command for UnitCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _user: &UserContext,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let value = BigDecimal::from_str(args.value.trim())?;
        let (prefix, base) = symbol::parse_symbol(&args.unit).unwrap();
        let base_unit = Unit {
            value: value.clone(),
            base,
            prefix,
            filter: UnitFilter::Always,
        };
        let output_unit = args.to.and_then(|to| symbol::parse_symbol(&to));
        let scale = prefix
            .map(|prefix| prefix.scale())
            .unwrap_or_else(BigDecimal::one);
        let mut converted = conversion::convert(value * scale, base);
        converted.sort_unstable_by_key(|unit| (unit.base, unit.prefix));
        let mut converted = converted
            .into_iter()
            .filter(|unit| unit.base != base_unit.base || unit.prefix != base_unit.prefix)
            .collect::<Vec<_>>();
        let mut filtered = if let Some((prefix, base)) = output_unit {
            converted
                .iter()
                .cloned()
                .filter(|item| item.base == base && item.prefix == prefix)
                .collect::<Vec<_>>()
        } else {
            converted
                .iter()
                .cloned()
                .filter(|unit| match unit.filter {
                    UnitFilter::Optional => false,
                    UnitFilter::MaximumScale(scale) => {
                        if let Some(value) = unit.value.to_f64() {
                            value.log10().abs() <= scale as f64
                        } else {
                            false
                        }
                    }
                    _ => true,
                })
                .collect::<Vec<_>>()
        };
        if filtered.is_empty() {
            converted.sort_by_key(|unit| unit.filter);
            converted
                .sort_by_key(|unit| unit.value.to_f64().unwrap_or(f64::MAX).log10().abs() as u64);
            filtered = converted.into_iter().take(1).collect::<Vec<_>>();
        }
        let converted = filtered
            .into_iter()
            .map(|unit| format!("`{}`", unit.normalized()))
            .collect::<Vec<_>>();
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/unit/")?;
        Ok(Output::new()
            .set_title("Unit Converter")
            .add_block(block::Data::new().set_highlighted_text_data(
                format!("{} =\n{}", base_unit, converted.join("\n")),
                &Default::default(),
            ))
            .add_metadata(docs))
    }

    fn priority(&self) -> i32 {
        -10
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    pub value: String,

    #[clap(short, long)]
    pub unit: String,

    #[clap(short, long)]
    pub to: Option<String>,
}
