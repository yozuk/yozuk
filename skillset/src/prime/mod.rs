#![forbid(unsafe_code)]
#![deny(clippy::all)]

use bigdecimal::{BigDecimal, ToPrimitive};
use clap::Parser;
use num_bigint::ToBigInt;
use std::str::FromStr;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"XCgzZKk_EgpTRQnDAvpwA",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(PrimeCorpus)
            .add_translator(PrimeTranslator)
            .set_command(PrimeCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct PrimeCorpus;

impl Corpus for PrimeCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!([
                "is",
                "0"; "input:number",
                "prime"; "keyword"
            ]),
            tk!([
                "is",
                "0"; "input:number",
                "a",
                "prime"; "keyword",
                "number"
            ]),
            tk!([
                "is",
                "65535"; "input:number",
                "prime"; "keyword"
            ]),
            tk!([
                "is",
                "65535"; "input:number",
                "a",
                "prime"; "keyword",
                "number"
            ]),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Debug)]
pub struct PrimeTranslator;

impl Translator for PrimeTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let prime = args
            .iter()
            .any(|arg| arg.tag == "keyword" && normalized_eq(arg.as_str(), &["prime"], 1));

        if prime {
            let inputs = args
                .iter()
                .filter(|arg| arg.tag == "input:number")
                .map(|arg| BigDecimal::from_str(arg.as_str()).ok())
                .collect::<Vec<_>>();
            if let [Some(num)] = &inputs[..] {
                return Some(CommandArgs::new().add_args(["--test".to_string(), num.to_string()]));
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct PrimeCommand;

impl Command for PrimeCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let primality = if let Some(num) = args.test.to_u32() {
            if prime_tools::is_u32_prime(num) {
                Primality::Yes
            } else {
                Primality::No
            }
        } else if let Some(num) = args.test.to_bigint() {
            if miller_rabin::is_prime(&num, 16) {
                Primality::Probably
            } else {
                Primality::No
            }
        } else {
            Primality::No
        };
        Ok(Output::new()
            .set_title("Primality Test")
            .add_block(block::Data::new().set_text_data(primality.to_string(&args.test)))
            .add_metadata(Metadata::value(primality != Primality::No)))
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Primality {
    Yes,
    Probably,
    No,
}

impl Primality {
    fn to_string(self, num: &BigDecimal) -> String {
        let num = num.to_string();
        match self {
            Self::Yes => format!("Yes, {num} is a prime number."),
            Self::Probably => format!("Yes, {num} is probably a prime number. (Miller–Rabin test)"),
            Self::No => format!("No, {num} is not a prime number."),
        }
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub test: BigDecimal,
}
