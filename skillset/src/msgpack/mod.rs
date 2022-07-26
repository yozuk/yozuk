use clap::Parser;
use yozuk_helper_encoding::EncodingPreprocessor;
use yozuk_sdk::{encoding::RawEncoding, prelude::*};

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"3jD5f89FcJbrsvFBB387r",
    init: |_| {
        Skill::builder()
            .add_preprocessor(EncodingPreprocessor::new(RawEncoding::all()))
            .add_translator(MsgpackTranslator)
            .set_command(MsgpackCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct MsgpackTranslator;

impl Translator for MsgpackTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let is_msgpack = !args.is_empty()
            && args.iter().all(|arg| {
                let mut rd = &arg.data[..];
                rmpv::decode::read_value_ref(&mut rd).is_ok() && rd.is_empty()
            });
        if is_msgpack {
            return Some(
                CommandArgs::new().add_args_iter(args.iter().map(|arg| base64::encode(&arg.data))),
            );
        }
        None
    }
}

#[derive(Debug)]
pub struct MsgpackCommand;

impl Command for MsgpackCommand {
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
            .filter_map(|arg| {
                base64::decode(arg.as_str()).ok().and_then(|data| {
                    let mut rd = &data[..];
                    rmpv::decode::read_value(&mut rd)
                        .ok()
                        .and_then(|value| serde_json::to_string_pretty(&value).ok())
                })
            })
            .flat_map(|json| {
                vec![
                    Block::Comment(block::Comment::new().set_text("Decoding MessagePack")),
                    Block::Data(
                        block::Data::new()
                            .set_text_data(json)
                            .set_media_type(media_type!(APPLICATION / JSON)),
                    ),
                ]
            });
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/msgpack/")?;
        Ok(Output::new()
            .set_title("Msgpack Decoder")
            .add_blocks_iter(blocks)
            .add_metadata(docs))
    }

    fn priority(&self) -> i32 {
        -50
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,
}
