use clap::Parser;
use itertools::iproduct;
use pix::gray::SGray8;
use qrcode_generator::QrCodeEcc;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"bAy_Du64C8aBhMa_VvVrm",
    init: |_| {
        Skill::builder()
            .add_corpus(QrCodeCorpus)
            .add_translator(QrCodeTranslator)
            .set_command(QrCodeCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct QrCodeCorpus;

impl Corpus for QrCodeCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        let inputs = vec![
            "Hello World",
            "😍😗😋",
            "quick brown fox jumps over the lazy dog",
            "Veterinarian",
        ];
        iproduct!(
            inputs.clone(),
            ["as", "to", "in", "into"],
            ["QR", "QRCode", "qrcode"]
        )
        .map(|(data, prefix, name)| {
            tk!([
                data; "input:data",
                prefix,
                name; "command"
            ])
        })
        .collect()
    }
}

#[derive(Debug)]
pub struct QrCodeTranslator;

impl Translator for QrCodeTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let command = args.iter().any(|arg| {
            arg.tag == "command" && normalized_eq(arg.as_str(), &["QR", "QRCode", "qrcode"], 0)
        });

        if command {
            let inputs = args
                .iter()
                .filter(|arg| arg.tag == "input:data")
                .map(|arg| arg.as_str());
            return Some(CommandArgs::new().add_args_iter(inputs));
        }

        None
    }
}

const IMAGE_MARGIN: usize = 4;
const IMAGE_SCALE: usize = 4;

#[derive(Debug)]
pub struct QrCodeCommand;

impl Command for QrCodeCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let blocks = args
            .inputs
            .iter()
            .filter_map(|arg| qrcode_generator::to_matrix(arg, QrCodeEcc::Low).ok())
            .flat_map(|code| {
                let image_size = (code.len() + IMAGE_MARGIN * 2) * IMAGE_SCALE;
                let mut pixels = vec![SGray8::new(255); image_size * image_size];
                for (y, bits) in code.iter().enumerate() {
                    for (x, bit) in bits.iter().enumerate() {
                        if *bit {
                            let x = (x + IMAGE_MARGIN) * IMAGE_SCALE;
                            let y = (y + IMAGE_MARGIN) * IMAGE_SCALE;
                            for rx in 0..IMAGE_SCALE {
                                for ry in 0..IMAGE_SCALE {
                                    let x = x + rx;
                                    let y = y + ry;
                                    pixels[x + y * image_size] = SGray8::new(0);
                                }
                            }
                        }
                    }
                }
                let raster = png_pong::PngRaster::Gray8(pix::Raster::with_pixels(
                    image_size as u32,
                    image_size as u32,
                    pixels,
                ));
                let mut out_data = Vec::new();
                let mut encoder = png_pong::Encoder::new(&mut out_data).into_step_enc();
                let step = png_pong::Step { raster, delay: 0 };
                encoder.encode(&step).unwrap();
                vec![Block::Data(
                    block::Data::new()
                        .set_data(out_data)
                        .set_file_name("qrcode.png")
                        .set_media_type(media_type!(IMAGE / PNG))
                        .set_display(DisplaySuggestion {
                            image: Some(ImageDisplay::Pixelated),
                            ..Default::default()
                        }),
                )]
            });
        Ok(Output::new()
            .set_title("QRCode Generator")
            .add_blocks_iter(blocks))
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,
}
