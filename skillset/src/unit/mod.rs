use bigdecimal::BigDecimal;
use bigdecimal::One;
use clap::Parser;
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
    model_id: b"FiANsDlUf9OI5fc3LTFA_",
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

#[derive(Debug)]
pub struct UnitSuggestions;

impl Suggestions for UnitSuggestions {
    fn suggestions(&self, seed: u64, _args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        let mut rng = StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(10..=1000);
        let unit = ["km", "in", "hPa", "kg", "oz.", "KiB", "mph", "°F"]
            .choose(&mut rng)
            .unwrap();
        vec![format!("Convert {}{}", n, unit)]
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
            .collect()
    }
}

#[derive(Debug)]
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
            if let [unit] = units[..] {
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

#[derive(Debug)]
pub struct UnitCommand;

impl Command for UnitCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let value = BigDecimal::from_str(args.value.trim())?;
        let (prefix, base) = symbol::parse_symbol(&args.unit).unwrap();
        let base_unit = Unit {
            value: value.clone(),
            base,
            prefix,
        };
        let scale = prefix
            .map(|prefix| prefix.scale())
            .unwrap_or_else(BigDecimal::one);
        let mut converted = conversion::convert(value * scale, base);
        converted.sort_unstable_by_key(|unit| (unit.base, unit.prefix));
        let converted = converted
            .into_iter()
            .filter(|unit| unit.base != base_unit.base || unit.prefix != base_unit.prefix)
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
}
