#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use itertools::iproduct;
use palette::{Hsla, Hsva, Hwba, IntoColor, Srgba};
use std::str::FromStr;
use yozuk_helper_preprocessor::{TokenMerger, TokenParser};
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"fYemWbybyq3U8v_aB9eWYc",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_preprocessor(TokenMerger::new(ColorTokenParser))
            .add_labeler(ColorLabeler)
            .add_corpus(ColorCorpus)
            .add_translator(ColorTranslator)
            .set_command(ColorCommand)
            .build()
    },
};

struct ColorTokenParser;

impl TokenParser for ColorTokenParser {
    fn parse(&self, tokens: &[Token]) -> Option<Token> {
        let exp = tokens
            .iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let mut tag = String::new();
        for token in tokens {
            if !token.tag.is_empty() {
                tag = token.tag.clone();
            }
        }
        css_color::Srgb::from_str(&exp).ok().map(|_| Token {
            data: exp.into(),
            media_type: "text/vnd.yozuk.color".parse().unwrap(),
            tag,
        })
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
                    .filter(|token| css_color::Srgb::from_str(token.as_str()).is_ok())
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
                .all(|arg| css_color::Srgb::from_str(arg.as_str()).is_ok())
        {
            return Some(
                CommandArgs::new().add_args_iter(inputs.into_iter().map(|arg| arg.as_str())),
            );
        }

        None
    }
}

#[derive(Debug)]
pub struct ColorCommand;

impl Command for ColorCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let (metadata, colors): (Vec<_>, Vec<_>) = args
            .inputs
            .iter()
            .filter_map(|color| css_color::Srgb::from_str(color).ok())
            .map(|color| {
                let color = Srgba::new(color.red, color.green, color.blue, color.alpha);
                (Metadata::value(hex_color(&color)), color)
            })
            .unzip();

        Ok(Output::new()
            .set_title("Color")
            .add_blocks_iter(colors.into_iter().flat_map(|color| render_color(&color)))
            .add_metadata_iter(metadata))
    }
}

fn hex_color(color: &Srgba) -> String {
    let rgba_u8: Srgba<u8> = (*color).into_format();
    if rgba_u8.alpha == 255 {
        format!(
            "#{:02x}{:02x}{:02x}",
            rgba_u8.color.red, rgba_u8.color.green, rgba_u8.color.blue
        )
    } else {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            rgba_u8.color.red, rgba_u8.color.green, rgba_u8.color.blue, rgba_u8.alpha
        )
    }
}

fn render_color(color: &Srgba) -> Vec<Block> {
    let mut colors = Vec::new();
    let rgba_u8: Srgba<u8> = (*color).into_format();

    colors.push(hex_color(color));

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
            "hsl({:.0} {:.0}% {:.0}%)",
            hsla.color.hue.to_positive_degrees(),
            hsla.color.saturation * 100.0,
            hsla.color.lightness * 100.0,
        )
    } else {
        format!(
            "hsl({:.0} {:.0}% {:.0}% / {})",
            hsla.color.hue.to_positive_degrees(),
            hsla.color.saturation * 100.0,
            hsla.color.lightness * 100.0,
            hsla.alpha
        )
    });

    let hwba: Hwba = (*color).into_color();
    colors.push(if hwba.alpha == 1.0 {
        format!(
            "hwb({:.0} {:.0}% {:.0}%)",
            hwba.color.hue.to_positive_degrees(),
            hwba.color.whiteness * 100.0,
            hwba.color.blackness * 100.0
        )
    } else {
        format!(
            "hwb({:.0} {:.0}% {:.0}% / {})",
            hwba.color.hue.to_positive_degrees(),
            hwba.color.whiteness * 100.0,
            hwba.color.blackness * 100.0,
            hwba.alpha
        )
    });

    let hsva: Hsva = (*color).into_color();
    colors.push(if hsva.alpha == 1.0 {
        format!(
            "hsv({:.0} {:.0}% {:.0}%)",
            hsva.color.hue.to_positive_degrees(),
            hsva.color.saturation * 100.0,
            hsva.color.value * 100.0,
        )
    } else {
        format!(
            "hsv({:.0} {:.0}% {:.0}% / {})",
            hsva.color.hue.to_positive_degrees(),
            hsva.color.saturation * 100.0,
            hsva.color.value * 100.0,
            hsva.alpha
        )
    });

    vec![
        Block::Preview(block::Preview::Color(block::ColorPreview {
            red: rgba_u8.color.red,
            green: rgba_u8.color.green,
            blue: rgba_u8.color.blue,
            alpha: rgba_u8.alpha,
        })),
        Block::Data(block::Data::new().set_text_data(colors.join("\n"))),
    ]
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,
}
