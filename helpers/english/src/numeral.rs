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
    let mut base = 1;
    for n in num.next()?.into_inner().rev() {
        sum += base
            * match n.as_str() {
                "one" | "a" => 1,
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
                "fourteen" => 14,
                "fifteen" => 15,
                "sixteen" => 16,
                "seventeen" => 17,
                "eighteen" => 18,
                "nineteen" => 19,
                "twenty" => 20,
                "thirty" => 30,
                "forty" => 40,
                "fifty" => 50,
                "sixty" => 60,
                "seventy" => 70,
                "eighty" => 80,
                "ninety" => 90,
                _ => 0,
            };
        match n.as_str() {
            "hundred" => {
                base *= 100;
            }
            "thousand" => {
                base = 1000;
            }
            _ => (),
        }
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
        assert_eq!(parse_numeral("three hundred forty seven"), Some(347));
        assert_eq!(parse_numeral("a hundred"), Some(100));
        assert_eq!(parse_numeral("a hundred and twelve"), Some(112));
        assert_eq!(parse_numeral("zero hundred"), None);
        assert_eq!(parse_numeral("forty seven hundred"), None);
        assert_eq!(parse_numeral("twenty eleven"), None);
        assert_eq!(parse_numeral("a thousand"), Some(1000));
        assert_eq!(parse_numeral("a thousand and twelve"), Some(1012));
        assert_eq!(parse_numeral("forty seven thousand"), Some(47000));
        assert_eq!(
            parse_numeral("three hundred forty seven thousand"),
            Some(347000)
        );
        assert_eq!(
            parse_numeral("three hundred forty seven thousand and three hundred forty seven"),
            Some(347347)
        );
        assert_eq!(
            parse_numeral("three hundred thousand and three hundred forty seven thousand"),
            None
        );
        assert_eq!(
            parse_numeral("two hundred and thirty four thousand, five hundred and sixty seven"),
            Some(234567)
        );
    }
}
