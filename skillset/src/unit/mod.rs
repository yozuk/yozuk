use bigdecimal::BigDecimal;
use bigdecimal::One;
use clap::Parser;
use std::str::FromStr;
use yozuk_sdk::prelude::*;

mod conversion;
mod entry;
mod symbol;
mod table;

use entry::*;
use table::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"FiANsDlUf9OI5fc3LTFA_",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_preprocessor(UnitPreprocessor)
            .add_corpus(UnitCorpus)
            .add_translator(UnitTranslator)
            .set_command(UnitCommand)
            .build()
    },
};

#[derive(Debug)]
struct UnitPreprocessor;

impl Preprocessor for UnitPreprocessor {
    fn preprocess(&self, input: Vec<Token>) -> Vec<Token> {
        input
            .into_iter()
            .flat_map(|token| {
                if let Some((num, unit)) = symbol::parse_num_symbol(token.as_utf8()) {
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
                                format!("{}{}", prefix.to_string(), sym); "unit:keyword"
                            ])
                        })
                        .chain(Some(tk!([
                            "1.0"; "input:value",
                            sym.to_string(); "unit:keyword"
                        ])))
                })
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct UnitTranslator;

impl Translator for UnitTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let values = args
            .iter()
            .filter(|arg| arg.tag == "input:value")
            .filter_map(|token| BigDecimal::from_str(token.as_utf8()).ok())
            .collect::<Vec<_>>();

        let units = args
            .iter()
            .filter(|arg| arg.tag == "unit:keyword")
            .filter(|arg| symbol::parse_symbol(arg.as_utf8()).is_some())
            .collect::<Vec<_>>();

        if let [value] = &values[..] {
            if let [unit] = units[..] {
                return Some(CommandArgs::new().add_args([
                    "--value".to_string(),
                    format!(" {}", value),
                    "--unit".to_string(),
                    unit.as_utf8().to_string(),
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
            .map(|unit| unit.normalized().to_string())
            .collect::<Vec<_>>();
        Ok(Output {
            title: "Unit Converter".into(),
            blocks: vec![Block::Data(
                block::Data::new()
                    .set_data(format!(
                        "{} =\n{}",
                        base_unit.to_string(),
                        converted.join("\n")
                    ))
                    .set_media_type(media_type!(TEXT / PLAIN)),
            )],
        })
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
