#![forbid(unsafe_code)]
#![deny(clippy::all)]

use itertools::iproduct;
use mediatype::media_type;
use std::collections::VecDeque;
use std::str::FromStr;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"fYemWbybyq3U8v_aB9eWYc",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_preprocessor(ColorPreprocessor)
            .add_labeler(ColorLabeler)
            .add_corpus(ColorCorpus)
            .add_translator(ColorTranslator)
            .set_command(ColorCommand)
            .build()
    },
};

#[derive(Debug)]
struct ColorPreprocessor;

impl Preprocessor for ColorPreprocessor {
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
                let exp = exp.join(" ");
                let is_exp = css_color::Srgb::from_str(&exp).is_ok();
                if is_exp {
                    for _ in 0..len {
                        tokens.pop_front();
                    }
                    output.push(tk!(exp, "text/vnd.yozuk.color"));
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

#[derive(Debug)]
pub struct ColorLabeler;

impl Labeler for ColorLabeler {
    fn label_features(&self, input: &[Token]) -> Vec<Vec<Feature>> {
        input
            .iter()
            .map(|token| {
                Some(token)
                    .filter(|token| css_color::Srgb::from_str(token.as_utf8()).is_ok())
                    .map(|_| Feature {
                        name: "expr:color".into(),
                        ..Default::default()
                    })
                    .into_iter()
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct ColorCorpus;

impl Corpus for ColorCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        iproduct!(
            [
                "#0f0",
                "rgb(0 255 0)",
                "rgb(0% 100% 0%)",
                "hsl(120deg 100% 50% / 100%)",
                "hwb(120 0% 0% / 1)",
                "lime"
            ],
            ["rgb", "hsl", "hwb"]
        )
        .flat_map(|(color, model)| {
            vec![
                tk!([color; "input:data"]),
                tk!([
                    color; "input:data",
                    "to",
                    model; "color:model"
                ]),
            ]
        })
        .collect()
    }
}

#[derive(Debug)]
pub struct ColorTranslator;

impl Translator for ColorTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let inputs = args
            .iter()
            .filter(|arg| arg.tag == "input:data")
            .collect::<Vec<_>>();

        if !inputs.is_empty()
            && inputs
                .iter()
                .all(|arg| css_color::Srgb::from_str(arg.as_utf8()).is_ok())
        {
            return Some(
                CommandArgs::new().add_args_iter(inputs.into_iter().map(|arg| arg.as_utf8())),
            );
        }

        None
    }
}

#[derive(Debug)]
pub struct ColorCommand;

impl Command for ColorCommand {
    fn run(&self, _args: CommandArgs, _streams: &mut [InputStream]) -> Result<Output, Output> {
        Ok(Output {
            sections: vec![Section::new("Hi. I'm Yozuk.", media_type!(TEXT / PLAIN))
                .kind(SectionKind::Comment)],
            ..Default::default()
        })
    }
}
