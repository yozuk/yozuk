#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use itertools::iproduct;
use mediatype::media_type;
use palette::{Hsla, Hsva, Hwba, IntoColor, Srgba};
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
                    let mut tag = String::new();
                    for _ in 0..len {
                        if let Some(token) = tokens.pop_front() {
                            if !token.tag.is_empty() {
                                tag = token.tag;
                            }
                        }
                    }
                    output.push(Token {
                        data: exp.into(),
                        media_type: "text/vnd.yozuk.color".parse().unwrap(),
                        tag,
                    });
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
        iproduct!([
            "#0f0",
            "rgb(0 255 0)",
            "rgb(0% 100% 0%)",
            "hsl(120deg 100% 50% / 100%)",
            "hwb(120 0% 0% / 1)",
            "lime"
        ])
        .flat_map(|color| vec![tk!([color; "input:data"])])
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
    fn run(&self, args: CommandArgs, _streams: &mut [InputStream]) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let colors = args
            .inputs
            .iter()
            .filter_map(|color| css_color::Srgb::from_str(color).ok())
            .map(|color| Srgba::new(color.red, color.green, color.blue, color.alpha));
        Ok(Output {
            sections: colors.map(|color| render_color(&color)).collect(),
            ..Default::default()
        })
    }
}

fn render_color(color: &Srgba) -> Section {
    let mut colors = Vec::new();
    let rgba_u8: Srgba<u8> = (*color).into_format();

    let hex = if rgba_u8.alpha == 255 {
        format!(
            "#{:02x}{:02x}{:02x}",
            rgba_u8.color.red, rgba_u8.color.green, rgba_u8.color.blue
        )
    } else {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            rgba_u8.color.red, rgba_u8.color.green, rgba_u8.color.blue, rgba_u8.alpha
        )
    };
    colors.push(hex.clone());

    colors.push(if color.alpha == 1.0 {
        format!(
            "rgb({} {} {})",
            rgba_u8.color.red, rgba_u8.color.green, rgba_u8.color.blue
        )
    } else {
        format!(
            "rgb({} {} {} / {})",
            rgba_u8.color.red, rgba_u8.color.green, rgba_u8.color.blue, rgba_u8.alpha
        )
    });

    let hsla: Hsla = (*color).into_color();
    colors.push(if hsla.alpha == 1.0 {
        format!(
            "hsl({} {}% {}%)",
            hsla.color.hue.to_positive_degrees(),
            hsla.color.saturation * 100.0,
            hsla.color.lightness * 100.0,
        )
    } else {
        format!(
            "hsl({} {}% {}% / {})",
            hsla.color.hue.to_positive_degrees(),
            hsla.color.saturation * 100.0,
            hsla.color.lightness * 100.0,
            hsla.alpha
        )
    });

    let hwba: Hwba = (*color).into_color();
    colors.push(if hwba.alpha == 1.0 {
        format!(
            "hwb({} {}% {}%)",
            hwba.color.hue.to_positive_degrees(),
            hwba.color.whiteness * 100.0,
            hwba.color.blackness * 100.0
        )
    } else {
        format!(
            "hwb({} {}% {}% / {})",
            hwba.color.hue.to_positive_degrees(),
            hwba.color.whiteness * 100.0,
            hwba.color.blackness * 100.0,
            hwba.alpha
        )
    });

    let hsva: Hsva = (*color).into_color();
    colors.push(if hsva.alpha == 1.0 {
        format!(
            "hsv({} {}% {}%)",
            hsva.color.hue.to_positive_degrees(),
            hsva.color.saturation * 100.0,
            hsva.color.value * 100.0,
        )
    } else {
        format!(
            "hsv({} {}% {}% / {})",
            hsva.color.hue.to_positive_degrees(),
            hsva.color.saturation * 100.0,
            hsva.color.value * 100.0,
            hsva.alpha
        )
    });

    Section::new(colors.join("\n"), media_type!(TEXT / PLAIN)).attr("com.yozuk.preview.color", hex)
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,
}
