use bigdecimal::{ToPrimitive, Zero};
use fraction::prelude::*;
use fraction::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use once_cell::sync::OnceCell;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::iter;
use thiserror::Error;
use yozuk_helper_english::normalized_eq;
use yozuk_helper_english::NumeralTokenParser;
use yozuk_helper_preprocessor::{TokenMerger, TokenParser};
use yozuk_sdk::prelude::*;

type Decimal = GenericDecimal<u64, u8>;
const DECIMAL_PRECISION: u8 = 16;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"Szq9sPvc3_bTUeMJSGjnC",
    init: |_| {
        Skill::builder()
            .add_corpus(DiceCorpus)
            .add_preprocessor(TokenMerger::new(NumeralTokenParser))
            .add_preprocessor(TokenMerger::new(DiceTokenParser))
            .add_translator(DiceTranslator)
            .add_suggests(DiceSuggests)
            .set_command(DiceCommand)
            .build()
    },
};

#[cfg(feature = "wild")]
const MAX_ROLLS: usize = u16::MAX as _;

#[cfg(not(feature = "wild"))]
const MAX_ROLLS: usize = 256;

#[derive(Debug)]
pub struct DiceSuggests;

impl Suggests for DiceSuggests {
    fn suggests(&self, seed: u64, _args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        let mut rng = StdRng::seed_from_u64(seed);
        let n: u32 = rng.gen_range(2..=10);
        vec![format!("Roll {} dice", n)]
    }
}

#[derive(Debug)]
pub struct DiceCorpus;

impl Corpus for DiceCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!(["dice"; "command"]),
            tk!(["🎲"; "command"]),
            tk!([
                "roll",
                "die"; "command"
            ]),
            tk!([
                "roll",
                "dice"; "command"
            ]),
            tk!([
                "roll",
                "3"; "input:count",
                "dice"; "command"
            ]),
        ]
    }
}

struct DiceTokenParser;

impl TokenParser for DiceTokenParser {
    fn parse(&self, tokens: &[Token]) -> Option<Token> {
        let exp = tokens
            .iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>()
            .join("");
        match DiceParser::parse(Rule::calculation, &exp) {
            Ok(pairs)
                if pairs
                    .clone()
                    .flatten()
                    .any(|pair| pair.as_rule() == Rule::dice) =>
            {
                Some(tk!(exp; "input:notation"))
            }
            _ => None,
        }
    }
}

#[derive(pest_derive::Parser)]
#[grammar = "dice/dice.pest"]
pub struct DiceParser;

fn eval(expression: Pairs<Rule>) -> Result<Value, DiceError> {
    use Assoc::*;

    static PREC_CLIMBER: OnceCell<PrecClimber<Rule>> = OnceCell::new();
    let climber = PREC_CLIMBER.get_or_init(|| {
        PrecClimber::new(vec![
            Operator::new(Rule::add, Left) | Operator::new(Rule::subtract, Left),
            Operator::new(Rule::multiply, Left) | Operator::new(Rule::divide, Left),
        ])
    });

    climber.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => Ok(Value::Sum(
                pair.as_str()
                    .parse::<Decimal>()
                    .map_err(|_| DiceError::Overflow)?,
            )),
            Rule::expr => eval(pair.into_inner()),
            Rule::dice => {
                let (rolls, size) = pair.as_str().split_once('d').unwrap();
                let rolls = rolls.parse::<usize>().map_err(|_| DiceError::Overflow)?;
                if rolls > MAX_ROLLS {
                    return Err(DiceError::TooManyRolls { limit: MAX_ROLLS });
                }

                let size = size.parse::<usize>().unwrap_or(6);
                let mut csrng = rand::thread_rng();
                let dice = iter::repeat(())
                    .take(rolls)
                    .map(|_| Decimal::from(csrng.gen_range(1..=size)))
                    .collect();
                Ok(Value::Dice(dice))
            }
            _ => unreachable!(),
        },
        |lhs: Result<Value, DiceError>, op: Pair<Rule>, rhs: Result<Value, DiceError>| {
            let lhs = lhs?.sum();
            let rhs = rhs?.sum();
            Ok(Value::Sum(match op.as_rule() {
                Rule::add => lhs.checked_add(&rhs).ok_or(DiceError::Overflow)?,
                Rule::subtract => lhs.checked_sub(&rhs).ok_or(DiceError::Overflow)?,
                Rule::multiply => lhs.checked_mul(&rhs).ok_or(DiceError::Overflow)?,
                Rule::divide if rhs.is_zero() => return Err(DiceError::DivisionByZero),
                Rule::divide => lhs.checked_div(&rhs).ok_or(DiceError::Overflow)?,
                _ => unreachable!(),
            }))
        },
    )
}

#[derive(Debug, Clone)]
enum Value {
    Dice(Vec<Decimal>),
    Sum(Decimal),
}

impl Value {
    fn sum(&self) -> Decimal {
        match self {
            Self::Dice(dice) => dice.iter().sum(),
            Self::Sum(sum) => *sum,
        }
    }

    fn metadata(&self) -> Vec<Metadata> {
        match self {
            Self::Dice(dice) => dice
                .iter()
                .filter_map(|val| val.to_u64())
                .map(Metadata::value)
                .collect(),
            Self::Sum(sum) => sum.to_f64().map(Metadata::value).into_iter().collect(),
        }
    }

    fn calc_precision(self) -> Self {
        match self {
            Self::Dice(dice) => Self::Dice(dice),
            Self::Sum(sum) => Self::Sum(sum.calc_precision(Some(DECIMAL_PRECISION))),
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Dice(dice) if dice.len() == 1 => {
                format!("🎲 {}", self.sum())
            }
            Self::Dice(dice) => {
                let history = dice.iter().map(ToString::to_string).collect::<Vec<_>>();
                format!("🎲 {}\nsum: {}", history.join(" "), self.sum())
            }
            Self::Sum(sum) => sum.to_string(),
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum DiceError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Too many rolls (limit: {limit})")]
    TooManyRolls { limit: usize },

    #[error("Overflow")]
    Overflow,
}

#[derive(Debug)]
pub struct DiceTranslator;

impl Translator for DiceTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let count = args
            .iter()
            .find(|arg| arg.tag == "input:count")
            .and_then(|arg| arg.as_str().parse::<usize>().ok())
            .unwrap_or(1);

        let commands = args
            .iter()
            .filter(|arg| arg.tag == "command")
            .collect::<Vec<_>>();

        if let [dice] = commands[..] {
            if normalized_eq(dice.as_str(), &["dice", "die", "🎲"], 0) {
                return Some(CommandArgs::new().add_args([format!("{}d6", count)]));
            }
        }

        if args.iter().any(|arg| arg.tag != "input:notation") {
            return None;
        }
        let exp = args
            .iter()
            .filter(|arg| arg.tag == "input:notation")
            .map(|arg| arg.as_str())
            .collect::<Vec<_>>();
        if exp.len() == 1 {
            Some(CommandArgs::new().add_args([exp[0]]))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct DiceCommand;

impl Command for DiceCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let rule = DiceParser::parse(Rule::calculation, &args.args[1])?;
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/dice/")?;
        Ok(eval(rule)
            .map(|result| {
                let result = result.calc_precision();
                Output::new()
                    .set_title("Dice")
                    .add_block(block::Data::new().set_text_data(result.to_string()))
                    .add_metadata_iter(result.metadata())
                    .add_metadata(docs.clone())
            })
            .map_err(|err| {
                Output::new()
                    .set_title("Dice")
                    .add_block(block::Comment::new().set_text(format!("{}", err)))
                    .add_metadata(docs)
            })?)
    }
}
