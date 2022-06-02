#![cfg(all(feature = "rpc", not(target_arch = "wasm32")))]

use anyhow::Result;
use json_rpc2::{Request, Response, Server, Service};
use mediatype::media_type;
use serde_derive::{Deserialize, Serialize};
use serde_json::{de::IoRead, Deserializer};
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

struct ServiceHandler {
    streams: Mutex<Vec<InputStream>>,
}

impl Service for ServiceHandler {
    type Data = Yozuk;
    fn handle(&self, request: &Request, zuk: &Self::Data) -> json_rpc2::Result<Option<Response>> {
        let response = match request.method() {
            "set_streams" => {
                let mut streams = self.streams.lock().unwrap();
                let req: Vec<Stream> = request.deserialize()?;
                *streams = req
                    .into_iter()
                    .filter_map(|data| match data {
                        Stream::Base64 { base64 } => base64::decode(base64).ok().map(|data| {
                            InputStream::new(
                                Cursor::new(data),
                                media_type!(APPLICATION / OCTET_STREAM),
                            )
                        }),
                        Stream::File { path } => File::open(path).ok().map(|file| {
                            InputStream::new(file, media_type!(APPLICATION / OCTET_STREAM))
                        }),
                    })
                    .collect();
                Some((request, serde_json::Value::Null).into())
            }
            "get_commands" => {
                let streams = self.streams.lock().unwrap();
                let req: GetCommandsRequest = request.deserialize()?;
                let tokens: Vec<Token> = req.into();
                let commands = zuk.get_commands(&tokens, &streams);
                let res = GetCommandsResponse { commands };
                Some((request, serde_json::to_value(res).unwrap()).into())
            }
            "run_commands" => {
                let mut streams = self.streams.lock().unwrap();
                let req: RunCommandsRequest = request.deserialize()?;
                let result: RunCommandsResponse = zuk
                    .run_commands(req.commands, &mut streams, Some(&req.i18n))
                    .into();
                streams.clear();
                Some((request, serde_json::to_value(result).unwrap()).into())
            }
            _ => None,
        };
        Ok(response)
    }
}

pub fn start_server<R, W>(zuk: Yozuk, reader: R, mut writer: W) -> Result<()>
where
    R: Read,
    W: Write,
{
    let service: Box<dyn Service<Data = Yozuk>> = Box::new(ServiceHandler {
        streams: Mutex::new(Vec::new()),
    });
    let server = Server::new(vec![&service]);
    let reader = IoRead::new(reader);
    let stream = Deserializer::new(reader).into_iter::<Request>();
    for request in stream {
        let response = server.serve(&request?, &zuk);
        serde_json::to_writer(&mut writer, &response)?;
        writeln!(&mut writer)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Stream {
    Base64 { base64: String },
    File { path: PathBuf },
}

#[derive(Default, Serialize, Deserialize)]
pub struct GetCommandsRequest {
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub input_tokens: Option<Vec<Token>>,
}

impl From<GetCommandsRequest> for Vec<Token> {
    fn from(mut req: GetCommandsRequest) -> Self {
        req.input_tokens
            .take()
            .unwrap_or_else(|| Tokenizer::new().tokenize(&req.input))
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetCommandsResponse {
    pub commands: Vec<CommandArgs>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct RunCommandsRequest {
    pub commands: Vec<CommandArgs>,
    #[serde(default)]
    pub i18n: I18n,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "result", content = "outputs")]
pub enum RunCommandsResponse {
    #[serde(rename = "ok")]
    Ok(Vec<Output>),
    #[serde(rename = "error")]
    Err(Vec<Output>),
}

impl From<std::result::Result<Vec<Output>, Vec<Output>>> for RunCommandsResponse {
    fn from(result: std::result::Result<Vec<Output>, Vec<Output>>) -> Self {
        match result {
            Ok(outputs) => Self::Ok(outputs),
            Err(outputs) => Self::Err(outputs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::Serialize;
    use serde_json::json;
    use std::io::Cursor;
    use tempfile::NamedTempFile;

    #[derive(Serialize)]
    struct Response {
        jsonrpc: String,
        id: usize,
        result: serde_json::Value,
    }

    impl Response {
        fn new<T>(id: usize, result: T) -> serde_json::Result<Self>
        where
            T: serde::Serialize,
        {
            Ok(Self {
                jsonrpc: "2.0".into(),
                id,
                result: serde_json::to_value(result)?,
            })
        }
    }

    #[test]
    fn test_rpc() {
        let mut input = Vec::<u8>::new();

        let command = GetCommandsRequest {
            input: "1 + 1".into(),
            ..Default::default()
        };
        let req = json_rpc2::Request::new(
            Some(json!(1u32)),
            "get_commands".into(),
            Some(serde_json::to_value(command).unwrap()),
        );
        input.append(&mut serde_json::to_vec(&req).unwrap());

        let command = GetCommandsRequest {
            input_tokens: Some(tk!(["2", "*", "3"])),
            ..Default::default()
        };
        let req = json_rpc2::Request::new(
            Some(json!(2u32)),
            "get_commands".into(),
            Some(serde_json::to_value(command).unwrap()),
        );
        input.append(&mut serde_json::to_vec(&req).unwrap());

        let command = RunCommandsRequest {
            commands: vec![CommandArgs::new().add_args(["yozuk-skill-calc", "1+1"])],
            ..Default::default()
        };
        let req = json_rpc2::Request::new(
            Some(json!(3u32)),
            "run_commands".into(),
            Some(serde_json::to_value(command).unwrap()),
        );
        input.append(&mut serde_json::to_vec(&req).unwrap());

        let streams = vec![Stream::Base64 {
            base64: "SGVsbG8gd29ybGQ=".into(),
        }];
        let req = json_rpc2::Request::new(
            Some(json!(4u32)),
            "set_streams".into(),
            Some(serde_json::to_value(streams).unwrap()),
        );
        input.append(&mut serde_json::to_vec(&req).unwrap());

        let command = RunCommandsRequest {
            commands: vec![CommandArgs::new().add_args([
                "yozuk-skill-digest",
                "--algorithm",
                "sha1",
            ])],
            ..Default::default()
        };
        let req = json_rpc2::Request::new(
            Some(json!(5u32)),
            "run_commands".into(),
            Some(serde_json::to_value(command).unwrap()),
        );
        input.append(&mut serde_json::to_vec(&req).unwrap());

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "Hello World?").unwrap();
        let path = file.into_temp_path().keep().unwrap();
        let streams = vec![Stream::File { path }];
        let req = json_rpc2::Request::new(
            Some(json!(6u32)),
            "set_streams".into(),
            Some(serde_json::to_value(streams).unwrap()),
        );
        input.append(&mut serde_json::to_vec(&req).unwrap());

        let command = RunCommandsRequest {
            commands: vec![CommandArgs::new().add_args([
                "yozuk-skill-digest",
                "--algorithm",
                "sha1",
            ])],
            ..Default::default()
        };
        let req = json_rpc2::Request::new(
            Some(json!(7u32)),
            "run_commands".into(),
            Some(serde_json::to_value(command).unwrap()),
        );
        input.append(&mut serde_json::to_vec(&req).unwrap());

        let zuk = Yozuk::builder().build();
        let mut input = Cursor::new(input);
        let mut output = Vec::<u8>::new();
        start_server(zuk, &mut input, &mut output).unwrap();

        let responses = vec![
            Response::new(
                1,
                GetCommandsResponse {
                    commands: vec![CommandArgs::new().add_args(["yozuk-skill-calc", "1+1"])],
                },
            ),
            Response::new(
                2,
                GetCommandsResponse {
                    commands: vec![CommandArgs::new().add_args(["yozuk-skill-calc", "2*3"])],
                },
            ),
            Response::new(
                3,
                RunCommandsResponse::Ok(vec![Output::new()
                    .set_title("Calculator")
                    .add_block(block::Data::new().set_text_data("2"))
                    .add_metadata(Metadata::value(2.0))]),
            ),
            Response::new(4, serde_json::Value::Null),
            Response::new(
                5,
                RunCommandsResponse::Ok(vec![Output::new().set_title("Digest").add_block(
                    block::Data::new().set_text_data("7b502c3a1f48c8609ae212cdfb639dee39673f5e"),
                )]),
            ),
            Response::new(6, serde_json::Value::Null),
            Response::new(
                7,
                RunCommandsResponse::Ok(vec![Output::new().set_title("Digest").add_block(
                    block::Data::new().set_text_data("357e04830e05f3c37ca86e491dce8acfa447efeb"),
                )]),
            ),
        ];

        let responses = responses
            .into_iter()
            .map(|res| serde_json::to_string(&res.unwrap()).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            responses.join("\n") + "\n"
        );
    }
}
