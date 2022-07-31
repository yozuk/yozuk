use bigdecimal::{ToPrimitive, Zero};
use fraction::prelude::*;
use fraction::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use once_cell::sync::OnceCell;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use thiserror::Error;
use yozuk_helper_english::NumeralTokenParser;
use yozuk_sdk::prelude::*;
use yozuk_sdk::preprocessor::{TokenMerger, TokenParser};

mod function;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"Bk4CKgQi8qhO3A0IBqK5t",
    init: |_| {
        Skill::builder()
            .add_suggestions(CalcSuggestions)
            .add_preprocessor(TokenMerger::new(NumeralTokenParser))
            .add_preprocessor(TokenMerger::new(CalcTokenParser))
            .add_translator(CalcTranslator)
            .set_command(CalcCommand)
            .build()
    },
};

type Decimal = GenericDecimal<u128, u8>;
const DECIMAL_PRECISION: u8 = 16;

pub struct CalcSuggestions;

impl Suggestions for CalcSuggestions {
    fn suggestions(&self, seed: u64, _args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        let mut rng = StdRng::seed_from_u64(seed);
        let operands = [
            "atan2(0.5, 0.2)",
            "25.0",
            "sin(1.2)",
            "sqrt(2)",
            "100",
            "log2(256)",
            "1",
            "abs(-8)",
            "123.45",
        ]
        .choose_multiple(&mut rng, 2)
        .collect::<Vec<_>>();
        vec![format!("{} + {}", operands[0], operands[1])]
    }
}

struct CalcTokenParser;

impl TokenParser for CalcTokenParser {
    fn parse(&self, tokens: &[Token]) -> Option<Token> {
        let exp = tokens
            .iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>()
            .join("");

        if CalcParser::parse(Rule::calculation, &exp).is_ok()
            && (tokens.len() >= 2 || CalcParser::parse(Rule::single_num, &exp).is_err())
        {
            Some(tk!(exp; "input:exp"))
        } else {
            None
        }
    }
}

#[derive(pest_derive::Parser)]
#[grammar = "calc/calc.pest"]
pub struct CalcParser;

fn eval(expression: Pairs<Rule>) -> Result<Decimal, CalcError> {
    use Assoc::*;

    static PREC_CLIMBER: OnceCell<PrecClimber<Rule>> = OnceCell::new();
    let climber = PREC_CLIMBER.get_or_init(|| {
        PrecClimber::new(vec![
            Operator::new(Rule::shr, Left) | Operator::new(Rule::shl, Left),
            Operator::new(Rule::add, Left) | Operator::new(Rule::subtract, Left),
            Operator::new(Rule::multiply, Left) | Operator::new(Rule::divide, Left),
            Operator::new(Rule::power, Right),
        ])
    });

    climber.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => Ok(pair
                .as_str()
                .parse::<Decimal>()
                .map_err(|_| CalcError::Overflow)?),
            Rule::func => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let func = match function::TABLE.get(name) {
                    Some(func) => func,
                    None => return Err(CalcError::NoSuchMethod(name.into())),
                };
                let mut args = Vec::new();
                for arg in inner.map(|expr| eval(expr.into_inner())) {
                    args.push(arg?);
                }
                func(&args)
            }
            Rule::expr => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: Result<Decimal, CalcError>, op: Pair<Rule>, rhs: Result<Decimal, CalcError>| {
            let lhs = lhs?;
            let rhs = rhs?;
            Ok(match op.as_rule() {
                Rule::add => lhs.checked_add(&rhs).ok_or(CalcError::Overflow)?,
                Rule::subtract => lhs.checked_sub(&rhs).ok_or(CalcError::Overflow)?,
                Rule::multiply => lhs.checked_mul(&rhs).ok_or(CalcError::Overflow)?,
                Rule::divide if rhs.is_zero() => return Err(CalcError::DivisionByZero),
                Rule::divide => lhs.checked_div(&rhs).ok_or(CalcError::Overflow)?,
                Rule::power => (lhs.to_f64().unwrap().powf(rhs.to_f64().unwrap())).into(),
                Rule::shr => lhs
                    .to_i128()
                    .ok_or(CalcError::IntegerOnlyOperation)?
                    .checked_shr(rhs.to_u32().ok_or(CalcError::IntegerOnlyOperation)?)
                    .ok_or(CalcError::Overflow)?
                    .into(),
                Rule::shl => lhs
                    .to_i128()
                    .ok_or(CalcError::IntegerOnlyOperation)?
                    .checked_shl(rhs.to_u32().ok_or(CalcError::IntegerOnlyOperation)?)
                    .ok_or(CalcError::Overflow)?
                    .into(),
                _ => unreachable!(),
            })
        },
    )
}

#[derive(Error, Debug, Clone)]
pub enum CalcError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Overflow")]
    Overflow,

    #[error("No such method: {0}")]
    NoSuchMethod(String),

    #[error("Operands must be integral")]
    IntegerOnlyOperation,

    #[error("Wrong number of arguments: expected {expected} but given {given}")]
    WrongNumberOfArguments { expected: usize, given: usize },
}

pub struct CalcTranslator;

impl Translator for CalcTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        if args.iter().any(|arg| arg.tag != "input:exp") {
            return None;
        }
        let exp = args
            .iter()
            .filter(|arg| arg.tag == "input:exp")
            .map(|arg| arg.as_str())
            .collect::<Vec<_>>();
        if exp.len() == 1 {
            Some(CommandArgs::new().add_args([exp[0]]))
        } else {
            None
        }
    }
}

pub struct CalcCommand;

impl Command for CalcCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let rule = CalcParser::parse(Rule::calculation, &args.args[1])?;
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/calc/")?;
        Ok(eval(rule)
            .map(|result| {
                let result = result.calc_precision(Some(DECIMAL_PRECISION));
                Output::new()
                    .set_title("Calculator")
                    .add_block(block::Data::new().set_text_data(format!("{}", result)))
                    .add_metadata(docs.clone())
            })
            .map_err(|err| {
                Output::new()
                    .set_title("Calculator")
                    .add_block(block::Comment::new().set_text(format!("{}", err)))
                    .add_metadata(docs)
            })?)
    }

    fn priority(&self) -> i32 {
        -50
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        let result = CalcCommand.run(
            CommandArgs::new().add_args(["", "(234 + 4343) * (23 - 43 / 4 * 5)"]),
            &mut [],
            &Default::default(),
        );
        let expected = Ok(Output::new()
            .set_title("Calculator")
            .add_block(block::Data::new().set_text_data("-140742.75"))
            .add_metadata(Metadata::docs("https://docs.yozuk.com/docs/skills/calc/").unwrap()));
        assert_eq!(result, expected);

        let result = CalcCommand.run(
            CommandArgs::new().add_args(["", "234 / (5 - 5)"]),
            &mut [],
            &Default::default(),
        );
        let expected = Err(CommandError::Output(
            Output::new()
                .set_title("Calculator")
                .add_block(block::Comment::new().set_text("Division by zero"))
                .add_metadata(Metadata::docs("https://docs.yozuk.com/docs/skills/calc/").unwrap()),
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_functions() {
        let result = CalcCommand.run(
            CommandArgs::new().add_args(["", "sin(0) - cos(atan2(0, 0)) * tanh(0) + sqrt(81)"]),
            &mut [],
            &Default::default(),
        );
        let expected = Ok(Output::new()
            .set_title("Calculator")
            .add_block(block::Data::new().set_text_data("9"))
            .add_metadata(Metadata::docs("https://docs.yozuk.com/docs/skills/calc/").unwrap()));
        assert_eq!(result, expected);
    }
}
