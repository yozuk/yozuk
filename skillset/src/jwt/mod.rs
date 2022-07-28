use anyhow::Result;
use clap::Parser;
use mediatype::media_type;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"881SN07tdT529jbaAYLwX",
    init: |_| {
        Skill::builder()
            .add_translator(JwtTranslator)
            .set_command(JwtCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct JwtTranslator;

impl Translator for JwtTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let is_jwt = !args.is_empty() && args.iter().all(|arg| decode_jwt(arg.as_str()).is_ok());
        if is_jwt {
            return Some(CommandArgs::new().add_args_iter(args.iter().map(|arg| arg.as_str())));
        }
        None
    }
}

#[derive(Debug)]
pub struct JwtCommand;

impl Command for JwtCommand {
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
            .filter_map(|arg| decode_jwt(arg).ok())
            .flat_map(|(header, claims, sig)| {
                vec![
                    Block::Data(
                        block::Data::new()
                            .set_text_data(serde_json::to_string_pretty(&header).unwrap())
                            .set_media_type(media_type!(APPLICATION / JSON)),
                    ),
                    Block::Data(
                        block::Data::new()
                            .set_text_data(serde_json::to_string_pretty(&claims).unwrap())
                            .set_media_type(media_type!(APPLICATION / JSON)),
                    ),
                    Block::Data(block::Data::new().set_data(sig)),
                ]
            });
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/jwt/")?;
        Ok(Output::new()
            .set_title("JWT Decoder")
            .add_blocks_iter(blocks)
            .add_metadata(docs))
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,
}

fn decode_jwt(data: &str) -> anyhow::Result<(Header, Claims, Vec<u8>)> {
    if let [header, claims, sig] = data.split('.').collect::<Vec<_>>()[..] {
        let header = base64::decode_config(header, base64::URL_SAFE)?;
        let claims = base64::decode_config(claims, base64::URL_SAFE)?;
        let sig = base64::decode_config(sig, base64::URL_SAFE)?;
        let header: Header = serde_json::from_slice(&header)?;
        let claims: Claims = serde_json::from_slice(&claims)?;
        Ok((header, claims, sig))
    } else {
        Err(anyhow::anyhow!("Invalid JWT token"))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Header {
    #[serde(skip_serializing_if = "Option::is_none")]
    alg: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    kid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    typ: Option<String>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Claims {
    #[serde(skip_serializing_if = "Option::is_none")]
    iss: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    iat: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    aud: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub: Option<String>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
