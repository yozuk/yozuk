use super::tk;
use super::token::*;
use pest::{iterators::Pair, Parser};

mod parser {
    #[derive(pest_derive::Parser)]
    #[grammar = "token.pest"]
    pub struct TokenParser;
}

use parser::*;

#[derive(Default)]
pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        if let Ok(args) = TokenParser::parse(Rule::args, input) {
            args.filter_map(parse_arg).collect()
        } else {
            input
                .split_whitespace()
                .map(|s| tk!(s.to_string()))
                .collect()
        }
    }
}

fn parse_arg(arg: Pair<Rule>) -> Option<Token> {
    let (raw_str, data) = match arg.as_rule() {
        Rule::string => (None, arg.as_str().to_string()),
        Rule::sq_string => (
            Some(arg.as_str().to_string()),
            arg.into_inner()
                .next()
                .unwrap()
                .as_str()
                .replace("\\'", "'"),
        ),
        Rule::dq_string => (
            Some(arg.as_str().to_string()),
            arg.into_inner()
                .next()
                .unwrap()
                .as_str()
                .replace("\\\"", "\""),
        ),
        _ => return None,
    };
    Some(Token {
        data: data.into(),
        raw_str,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokenizer = Tokenizer::new();
        assert_eq!(
            tokenizer.tokenize(" What's   the time "),
            tk!(["What's", "the", "time"])
        );
        assert_eq!(
            tokenizer.tokenize(r#" "Hello world" to md5 "#),
            vec![
                Token {
                    data: "Hello world".into(),
                    raw_str: Some("\"Hello world\"".into()),
                    ..Default::default()
                },
                tk!("to"),
                tk!("md5")
            ]
        );
        assert_eq!(
            tokenizer.tokenize(r#" (1 + 1) * 2 "#),
            tk!(["(1", "+", "1)", "*", "2"])
        );
        assert_eq!(
            tokenizer.tokenize(r#" " \" \" " "#),
            vec![Token {
                data: " \" \" ".into(),
                raw_str: Some(r#"" \" \" ""#.into()),
                ..Default::default()
            }]
        );
        assert_eq!(tokenizer.tokenize(" #ffffff "), tk!(["#ffffff"]));
        assert_eq!(
            tokenizer.tokenize(
                r#"　"Hello　world"
             to　md5　"#
            ),
            vec![
                Token {
                    data: "Hello　world".into(),
                    raw_str: Some("\"Hello　world\"".into()),
                    ..Default::default()
                },
                tk!("to"),
                tk!("md5")
            ]
        );
    }
}
