#![forbid(unsafe_code)]
#![deny(clippy::all)]

use bigdecimal::BigDecimal;
use bigdecimal::Zero;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use thiserror::Error;
use yozuk_helper_preprocessor::{TokenMerger, TokenParser};
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"Bk4CKgQi8qhO3A0IBqK5t",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_preprocessor(TokenMerger::new(CalcTokenParser))
            .add_translator(CalcTranslator)
            .set_command(CalcCommand)
            .build()
    },
};

struct CalcTokenParser;

impl TokenParser for CalcTokenParser {
    fn parse(&self, tokens: &[Token]) -> Option<Token> {
        let exp = tokens
            .iter()
            .map(|token| token.as_utf8())
            .collect::<Vec<_>>()
            .join("");

        if CalcParser::parse(Rule::calculation, &exp).is_ok()
            && (tokens.len() >= 2 || CalcParser::parse(Rule::single_num, &exp).is_err())
        {
            Some(tk!(exp, "text/vnd.yozuk.calc"))
        } else {
            None
        }
    }
}

#[derive(pest_derive::Parser)]
#[grammar = "calc/calc.pest"]
pub struct CalcParser;

lazy_static::lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
        ])
    };
}

fn eval(expression: Pairs<Rule>) -> Result<BigDecimal, CalcError> {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => Ok(pair.as_str().parse::<BigDecimal>().unwrap()),
            Rule::expr => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: Result<BigDecimal, CalcError>, op: Pair<Rule>, rhs: Result<BigDecimal, CalcError>| {
            let lhs = lhs?;
            let rhs = rhs?;
            Ok(match op.as_rule() {
                Rule::add => lhs + rhs,
                Rule::subtract => lhs - rhs,
                Rule::multiply => lhs * rhs,
                Rule::divide if rhs.is_zero() => return Err(CalcError::DivisionByZero),
                Rule::divide => lhs / rhs,
                _ => unreachable!(),
            })
        },
    )
}

#[derive(Error, Debug, Clone)]
pub enum CalcError {
    #[error("Division by zero")]
    DivisionByZero,
}

#[derive(Debug)]
pub struct CalcTranslator;

impl Translator for CalcTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let media_type = MediaType::parse("text/vnd.yozuk.calc").unwrap();
        if args.iter().any(|arg| arg.media_type != media_type) {
            return None;
        }
        let exp = args
            .iter()
            .filter(|arg| arg.media_type == media_type)
            .map(|arg| arg.as_utf8())
            .collect::<Vec<_>>();
        if exp.len() == 1 {
            Some(CommandArgs::new().add_args([exp[0]]))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct CalcCommand;

impl Command for CalcCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let rule = CalcParser::parse(Rule::calculation, &args.args[1])?;
        Ok(eval(rule)
            .map(|result| Output {
                title: "Calculator".into(),
                blocks: vec![Block::Data(
                    block::Data::new().set_text_data(format!("{}", result)),
                )],
                ..Default::default()
            })
            .map_err(|err| Output {
                title: "Calculator".into(),
                blocks: vec![Block::Comment(
                    block::Comment::new().set_text(format!("{}", err)),
                )],
                ..Default::default()
            })?)
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
        let expected = Ok(Output {
            title: "Calculator".into(),
            blocks: vec![Block::Data(block::Data::new().set_text_data("-140742.75"))],
            ..Default::default()
        });
        assert_eq!(result, expected);

        let result = CalcCommand.run(
            CommandArgs::new().add_args(["", "234 / (5 - 5)"]),
            &mut [],
            &Default::default(),
        );
        let expected = Err(CommandError::Output(Output {
            title: "Calculator".into(),
            blocks: vec![Block::Comment(
                block::Comment::new().set_text("Division by zero"),
            )],
            ..Default::default()
        }));
        assert_eq!(result, expected);
    }
}
