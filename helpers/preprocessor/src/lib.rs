#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::collections::VecDeque;
use std::fmt;
use yozuk_sdk::prelude::*;

pub trait TokenParser: Send + Sync {
    fn parse(&self, tokens: &[Token]) -> Option<Token>;
}

pub struct TokenMerger<P> {
    parser: P,
}

impl<P> fmt::Debug for TokenMerger<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenMerger").finish()
    }
}

impl<P> TokenMerger<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<P> Preprocessor for TokenMerger<P>
where
    P: TokenParser + 'static,
{
    fn preprocess(&self, input: Vec<Token>) -> Vec<Token> {
        let mut output = Vec::new();
        let mut tokens = input.into_iter().collect::<VecDeque<_>>();
        while !tokens.is_empty() {
            for i in 1..=tokens.len() {
                let len = tokens.len() + 1 - i;
                let exp = tokens.as_slices().0;
                let exp = if exp.len() >= len {
                    &exp[..len]
                } else {
                    &tokens.make_contiguous()[..len]
                };
                if let Some(merged) = self.parser.parse(exp) {
                    for _ in 0..len {
                        tokens.pop_front();
                    }
                    output.push(merged);
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
