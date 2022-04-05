use bigdecimal::BigDecimal;
use mediatype::media_type;
use std::str::FromStr;
use yozuk_sdk::prelude::*;

mod conversion;
mod symbol;
mod unit;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"86lRFe79o8JOiQCogjsXc",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(UnitCorpus)
            .add_translator(UnitTranslator)
            .set_command(UnitCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct UnitCorpus;

impl Corpus for UnitCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        symbol::ENTRIES
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
                return Some(CommandArgs::new().add_args([format!("{}{}", value, unit.as_utf8())]));
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
        _args: CommandArgs,
        _streams: &mut [InputStream],
    ) -> Result<Output, CommandError> {
        Ok(Output {
            module: "Unit Converter".into(),
            sections: vec![Section::new("unit", media_type!(TEXT / PLAIN))],
        })
    }

    fn priority(&self) -> i32 {
        -10
    }
}
