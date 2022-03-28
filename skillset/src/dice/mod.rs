#![forbid(unsafe_code)]
#![deny(clippy::all)]

use mediatype::{media_type, MediaType};
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde_derive::Deserialize;
use std::collections::VecDeque;
use std::iter;
use thiserror::Error;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"Szq9sPvc3_bTUeMJSGjnC",
    config_schema: Some(include_str!("./schema.json")),
    init: |_, config| {
        Skill::builder()
            .add_corpus(DiceCorpus)
            .add_preprocessor(DicePreprocessor)
            .add_translator(DiceTranslator)
            .set_command(DiceCommand(config.get()))
            .build()
    },
};

const MAX_ROLLS: usize = 100;

#[derive(Debug)]
pub struct DiceCorpus;

impl Corpus for DiceCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!(["dice"; "command:dice"]),
            tk!([
                "roll",
                "die"; "command:dice"
            ]),
            tk!([
                "roll",
                "dice"; "command:dice"
            ]),
            tk!([
                "roll",
                "3"; "input:count",
                "dice"; "command:dice"
            ]),
        ]
    }
}

#[derive(Debug)]
struct DicePreprocessor;

impl Preprocessor for DicePreprocessor {
    fn preprocess(&self, input: Vec<Token>) -> Vec<Token> {
        let mut output = Vec::new();
        let mut tokens = input.into_iter().collect::<VecDeque<_>>();
        while !tokens.is_empty() {
            for i in 1..=tokens.len() {
                let len = tokens.len() + 1 - i;
                let exp = tokens
                    .iter()
                    .take(len)
                    .map(|token| token.as_utf8())
                    .collect::<Vec<_>>();
                let exp = exp.join("");
                let is_exp = match DiceParser::parse(Rule::calculation, &exp) {
                    Ok(pairs) => pairs.flatten().any(|pair| pair.as_rule() == Rule::dice),
                    _ => false,
                };
                if is_exp {
                    for _ in 0..len {
                        tokens.pop_front();
                    }
                    output.push(tk!(exp, "text/vnd.yozuk.dice"));
                    break;
                }
            }
            if let Some(front) = tokens.pop_front() {
                output.push(front);
            }
        }
        output
    }
}

#[derive(pest_derive::Parser)]
#[grammar = "dice/dice.pest"]
pub struct DiceParser;

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

fn eval(expression: Pairs<Rule>, config: &DiceConfig) -> Result<Value, DiceError> {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::int => Ok(Value::Sum(pair.as_str().parse::<isize>().unwrap())),
            Rule::expr => eval(pair.into_inner(), config),
            Rule::dice => {
                let (rolls, size) = pair.as_str().split_once('d').unwrap();
                let rolls = rolls.parse::<usize>().unwrap();
                if rolls > MAX_ROLLS {
                    return Err(DiceError::TooManyRolls { limit: MAX_ROLLS });
                }

                let size = size.parse::<isize>().unwrap_or(6);
                let mut csrng = rand::thread_rng();
                let mut fastrng = SmallRng::from_entropy();
                let dice = iter::repeat(())
                    .take(rolls)
                    .map(|_| {
                        if config.secure {
                            csrng.gen_range(1..=size)
                        } else {
                            fastrng.gen_range(1..=size)
                        }
                    })
                    .collect();
                Ok(Value::Dice(dice))
            }
            _ => unreachable!(),
        },
        |lhs: Result<Value, DiceError>, op: Pair<Rule>, rhs: Result<Value, DiceError>| {
            let lhs = lhs?.sum();
            let rhs = rhs?.sum();
            Ok(Value::Sum(match op.as_rule() {
                Rule::add => lhs + rhs,
                Rule::subtract => lhs - rhs,
                Rule::multiply => lhs * rhs,
                Rule::divide if rhs == 0 => return Err(DiceError::DivisionByZero),
                Rule::divide => lhs / rhs,
                _ => unreachable!(),
            }))
        },
    )
}

#[derive(Debug, Clone)]
enum Value {
    Dice(Vec<isize>),
    Sum(isize),
}

impl Value {
    fn sum(&self) -> isize {
        match self {
            Self::Dice(dice) => dice.iter().sum(),
            Self::Sum(sum) => *sum,
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Dice(dice) if dice.len() == 1 => self.sum().to_string(),
            Self::Dice(dice) => {
                let history = dice.iter().map(ToString::to_string).collect::<Vec<_>>();
                format!("{} ({})", self.sum(), history.join(" "))
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
}

#[derive(Debug)]
pub struct DiceTranslator;

impl Translator for DiceTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let count = args
            .iter()
            .find(|arg| arg.tag == "input:count")
            .and_then(|arg| arg.as_utf8().parse::<usize>().ok())
            .unwrap_or(1);

        let commands = args
            .iter()
            .filter(|arg| arg.tag == "command:dice")
            .collect::<Vec<_>>();

        if let [dice] = commands[..] {
            if normalized_eq(dice.as_utf8(), &["dice", "die"], 0) {
                return Some(CommandArgs::new().add_args([format!("{}d6", count)]));
            }
        }

        let media_type = MediaType::parse("text/vnd.yozuk.dice").unwrap();
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
pub struct DiceCommand(DiceConfig);

impl Command for DiceCommand {
    fn run(&self, args: CommandArgs, _streams: &mut [InputStream]) -> Result<Output, Output> {
        let rule = DiceParser::parse(Rule::calculation, &args.args[1]).unwrap();
        eval(rule, &self.0)
            .map(|result| Output {
                module: "Dice".into(),
                sections: vec![Section::new(result.to_string(), media_type!(TEXT / PLAIN))],
            })
            .map_err(|err| Output {
                module: "Dice".into(),
                sections: vec![Section::new(format!("{}", err), media_type!(TEXT / PLAIN))
                    .kind(SectionKind::Comment)],
            })
    }
}

#[derive(Debug, Default, Copy, Clone, Deserialize)]
struct DiceConfig {
    #[serde(default)]
    secure: bool,
}
