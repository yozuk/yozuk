use pest::Parser;
use yozuk_helper_preprocessor::TokenParser;
use yozuk_sdk::prelude::*;

#[derive(pest_derive::Parser)]
#[grammar = "numeral.pest"]
struct NumeralParser;

pub fn parse_numeral(input: &str) -> Option<i32> {
    let input = input.to_ascii_lowercase();
    let mut num = NumeralParser::parse(Rule::num, &input).ok()?;
    let mut sum = 0;
    for n in num.next()?.into_inner() {
        sum += match n.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            "ten" => 10,
            "eleven" => 11,
            "twelve" => 12,
            "thirteen" => 13,
            "fourtteen" => 14,
            "fifteen" => 15,
            "sixteen" => 16,
            "seventeen" => 17,
            "eighteen" => 18,
            "nineteen" => 19,
            "twenty" => 20,
            "thirty" => 30,
            "fourty" => 40,
            "fifty" => 50,
            "sixty" => 60,
            "seventy" => 70,
            "eighty" => 80,
            "ninety" => 90,
            _ => 0,
        };
    }
    Some(sum)
}

pub struct NumeralTokenParser;

impl TokenParser for NumeralTokenParser {
    fn parse(&self, tokens: &[Token]) -> Option<Token> {
        let exp = tokens
            .iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        parse_numeral(&exp).map(|num| tk!(num.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numeral() {
        assert_eq!(parse_numeral("zero"), Some(0));
        assert_eq!(parse_numeral("five"), Some(5));
        assert_eq!(parse_numeral("sixteen"), Some(16));
        assert_eq!(parse_numeral("twenty"), Some(20));
        assert_eq!(parse_numeral("twenty-two"), Some(22));
        assert_eq!(parse_numeral("twenty two"), Some(22));
        assert_eq!(parse_numeral("twentytwo"), Some(22));
        assert_eq!(parse_numeral("Twenty Two"), Some(22));
        assert_eq!(parse_numeral("twenty eleven"), None);
    }
}
