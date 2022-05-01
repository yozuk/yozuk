use pest::{iterators::Pair, Parser};
use yozuk_sdk::prelude::*;

#[derive(pest_derive::Parser)]
#[grammar = "token.pest"]
pub struct TokenParser;

pub fn tokenize(input: &str) -> Vec<Token> {
    if let Ok(args) = TokenParser::parse(Rule::args, input) {
        args.filter_map(parse_arg).collect()
    } else {
        input
            .split_whitespace()
            .map(|s| tk!(s.to_string()))
            .collect()
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
        assert_eq!(
            tokenize(" What's   the time "),
            tk!(["What's", "the", "time"])
        );
        assert_eq!(
            tokenize(r#" "Hello world" to md5 "#),
            tk!(["Hello world", "to", "md5"])
        );
        assert_eq!(
            tokenize(r#" (1 + 1) * 2 "#),
            tk!(["(1", "+", "1)", "*", "2"])
        );
        assert_eq!(tokenize(r#" " \" \" " "#), tk!([" \" \" "]));
        assert_eq!(tokenize(" #ffffff "), tk!(["#ffffff"]));
    }
}
