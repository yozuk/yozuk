use clap::Parser;
use itertools::iproduct;
use palette::{Hsla, Hsva, Hwba, IntoColor, Srgba};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::str::FromStr;
use yozuk_sdk::prelude::*;
use yozuk_sdk::preprocessor::{TokenMerger, TokenParser};

mod keywords;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"Aiz9EnzXBdZcghXc9pwf~",
    init: |_| {
        Skill::builder()
            .add_suggestions(ColorSuggestions)
            .add_preprocessor(TokenMerger::new(ColorTokenParser))
            .add_labeler(ColorLabeler)
            .add_corpus(ColorCorpus)
            .add_translator(ColorTranslator)
            .set_command(ColorCommand)
            .build()
    },
};

pub struct ColorSuggestions;

impl Suggestions for ColorSuggestions {
    fn suggestions(&self, seed: u64, args: &[Token], streams: &[InputStream]) -> Vec<String> {
        if args.is_empty() && streams.is_empty() {
            let mut rng = StdRng::seed_from_u64(seed);
            let [r, g, b]: [u8; 3] = rng.gen();
            let h = rng.gen_range(0..360);
            let s = rng.gen_range(0..=100);
            let l = rng.gen_range(0..=100);
            let space = ["hsv", "hwb", "hsl", "hsla"].choose(&mut rng).unwrap();
            vec![
                format!("#{:02x}{:02x}{:02x} to {space}", r, g, b),
                format!("hsl({h} {s}% {l}%) to rgba"),
            ]
        } else {
            vec![]
        }
    }
}

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
            tag,
            ..Default::default()
        })
    }
}

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

pub struct ColorCorpus;

impl Corpus for ColorCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        let inputs = vec![
            "#0f0",
            "rgb(0 255 0)",
            "rgb(0% 100% 0%)",
            "hsl(120deg 100% 50% / 100%)",
            "hwb(120 0% 0% / 1)",
            "lime",
        ];
        inputs
            .clone()
            .into_iter()
            .flat_map(|color| vec![tk!([color; "input:data"])])
            .chain(
                iproduct!(
                    inputs.clone(),
                    ["as", "to", "in", "into"],
                    ["rgb", "rgba", "hsl", "hsla", "hwb", "hwba"]
                )
                .map(|(data, prep, space)| {
                    tk!([
                        data; "input:data",
                        prep,
                        space; "input:space"
                    ])
                }),
            )
            .collect()
    }
}

pub struct ColorTranslator;

impl Translator for ColorTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let inputs = args
            .iter()
            .filter(|arg| arg.tag == "input:data")
            .collect::<Vec<_>>();

        let spaces = args
            .iter()
            .filter(|arg| arg.tag == "input:space")
            .filter(|arg| COLOR_SPACES.contains_key(&arg.as_str().to_ascii_lowercase()))
            .collect::<Vec<_>>();

        if let [input] = &inputs[..] {
            if let [space] = &spaces[..] {
                return Some(
                    CommandArgs::new()
                        .add_args(["--space", space.as_str()])
                        .add_args([input.as_str()]),
                );
            }
        }

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

pub struct ColorCommand;

impl Command for ColorCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _user: &UserContext,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;

        let (metadata, colors): (Vec<_>, Vec<_>) = args
            .inputs
            .iter()
            .filter_map(|color| css_color::Srgb::from_str(color).ok())
            .map(|color| {
                let color = Srgba::new(color.red, color.green, color.blue, color.alpha);
                (hex_color(&color), color)
            })
            .unzip();

        let space = args
            .space
            .and_then(|space| COLOR_SPACES.get(&space.to_ascii_lowercase()));

        let blocks: Vec<Block> = if let Some(space) = space {
            colors
                .iter()
                .map(space)
                .map(|s| Block::Data(block::Data::new().set_text_data(s)))
                .collect()
        } else {
            colors
                .into_iter()
                .flat_map(|color| render_color(&color))
                .collect()
        };

        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/color/")?;
        Ok(Output::new()
            .set_title("Color Converter")
            .add_blocks_iter(blocks)
            .add_metadata_iter(
                metadata
                    .into_iter()
                    .flat_map(|color| [Metadata::value(color.clone()), Metadata::color(color)]),
            )
            .add_metadata(docs))
    }
}

pub static COLOR_SPACES: phf::Map<&'static str, fn(&Srgba) -> String> = phf::phf_map! {
    "rgb" => |color| {
        let rgba_u8: Srgba<u8> = (*color).into_format();
        format!(
            "#{:02x}{:02x}{:02x}\nrgb({} {} {})",
            rgba_u8.color.red, rgba_u8.color.green, rgba_u8.color.blue,
            rgba_u8.color.red, rgba_u8.color.green, rgba_u8.color.blue
        )
    },
    "rgba" => |color| {
        let rgba_u8: Srgba<u8> = (*color).into_format();
        format!(
            "#{:02x}{:02x}{:02x}{:02x}\nrgb({} {} {} / {})",
            rgba_u8.color.red, rgba_u8.color.green, rgba_u8.color.blue, rgba_u8.alpha,
            rgba_u8.color.red, rgba_u8.color.green, rgba_u8.color.blue, rgba_u8.alpha
        )
    },
    "hsl" => |color| {
        let hsla: Hsla = (*color).into_color();
        format!(
            "hsl({:.0} {:.0}% {:.0}%)",
            hsla.color.hue.to_positive_degrees(),
            hsla.color.saturation * 100.0,
            hsla.color.lightness * 100.0,
        )
    },
    "hsla" => |color| {
        let hsla: Hsla = (*color).into_color();
        format!(
            "hsl({:.0} {:.0}% {:.0}% / {})",
            hsla.color.hue.to_positive_degrees(),
            hsla.color.saturation * 100.0,
            hsla.color.lightness * 100.0,
            hsla.alpha
        )
    },
    "hwb" => |color| {
        let hwba: Hwba = (*color).into_color();
        format!(
            "hwb({:.0} {:.0}% {:.0}%)",
            hwba.color.hue.to_positive_degrees(),
            hwba.color.whiteness * 100.0,
            hwba.color.blackness * 100.0
        )
    },
    "hwba" => |color| {
        let hwba: Hwba = (*color).into_color();
        format!(
            "hwb({:.0} {:.0}% {:.0}% / {})",
            hwba.color.hue.to_positive_degrees(),
            hwba.color.whiteness * 100.0,
            hwba.color.blackness * 100.0,
            hwba.alpha
        )
    },
    "hsv" => |color| {
        let hsva: Hsva = (*color).into_color();
        format!(
            "hsv({:.0} {:.0}% {:.0}%)",
            hsva.color.hue.to_positive_degrees(),
            hsva.color.saturation * 100.0,
            hsva.color.value * 100.0,
        )
    },
    "hsva" => |color| {
        let hsva: Hsva = (*color).into_color();
        format!(
            "hsv({:.0} {:.0}% {:.0}% / {})",
            hsva.color.hue.to_positive_degrees(),
            hsva.color.saturation * 100.0,
            hsva.color.value * 100.0,
            hsva.alpha
        )
    }
};

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

    let color_u32 =
        ((rgba_u8.red as u32) << 16) + ((rgba_u8.green as u32) << 8) + rgba_u8.blue as u32;
    if let Some(name) = keywords::KEYWORDS.get(&color_u32) {
        colors.push(name.to_string());
    }

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

    vec![Block::Data(
        block::Data::new().set_text_data(colors.join("\n")),
    )]
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,

    #[clap(long)]
    pub space: Option<String>,
}
