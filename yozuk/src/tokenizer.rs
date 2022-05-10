use pest::{iterators::Pair, Parser};
use yozuk_sdk::prelude::*;

#[derive(pest_derive::Parser)]
#[grammar = "token.pest"]
struct TokenParser;

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
    let token = match arg.as_rule() {
        Rule::string => arg.as_str().to_string(),
        Rule::sq_string => arg
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .replace("\\'", "'"),
        Rule::dq_string => arg
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .replace("\\\"", "\""),
        _ => return None,
    };
    Some(tk!(token))
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
            tk!(["Hello world", "to", "md5"])
        );
        assert_eq!(
            tokenizer.tokenize(r#" (1 + 1) * 2 "#),
            tk!(["(1", "+", "1)", "*", "2"])
        );
        assert_eq!(tokenizer.tokenize(r#" " \" \" " "#), tk!([" \" \" "]));
        assert_eq!(tokenizer.tokenize(" #ffffff "), tk!(["#ffffff"]));
    }
}
