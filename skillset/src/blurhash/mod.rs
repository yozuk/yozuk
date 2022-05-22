#![forbid(unsafe_code)]
#![deny(clippy::all)]

use clap::Parser;
use mediatype::media_type;
use pix::rgb::SRgba8;
use yozuk_sdk::prelude::*;

mod base83;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"DAyoatvv8aj-BkVUo-18M",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_translator(BlurHashTranslator)
            .set_command(BlurHashCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct BlurHashTranslator;

impl Translator for BlurHashTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let is_blurhash = args.iter().all(|arg| base83::validate_blurhash(&arg.data));
        if is_blurhash {
            return Some(CommandArgs::new().add_args_iter(args.iter().map(|arg| arg.as_str())));
        }
        None
    }
}

const IMAGE_SIZE: u32 = 64;

#[derive(Debug)]
pub struct BlurHashCommand;

impl Command for BlurHashCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let blocks = args.inputs.iter().flat_map(|arg| {
            let pixels = blurhash::decode(arg.as_str(), IMAGE_SIZE, IMAGE_SIZE, 1.0);
            let pixels = pixels
                .chunks(4)
                .map(|color| SRgba8::new(color[0], color[1], color[2], color[3]))
                .collect::<Vec<_>>();
            let raster = png_pong::PngRaster::Rgba8(pix::Raster::with_pixels(
                IMAGE_SIZE, IMAGE_SIZE, pixels,
            ));
            let mut out_data = Vec::new();
            let mut encoder = png_pong::Encoder::new(&mut out_data).into_step_enc();
            let step = png_pong::Step { raster, delay: 0 };
            encoder.encode(&step).unwrap();
            vec![
                Block::Comment(block::Comment::new().set_text("Decoding BlurHash")),
                Block::Data(
                    block::Data::new()
                        .set_data(out_data)
                        .set_file_name("blurhash.png")
                        .set_media_type(media_type!(IMAGE / PNG)),
                ),
            ]
        });
        Ok(Output::new()
            .set_title("BlurHash Decoder")
            .add_blocks_iter(blocks))
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,
}
