#![cfg(feature = "rpc")]

use anyhow::Result;
use json_rpc2::{Request, Response, Server, Service};
use serde_derive::{Deserialize, Serialize};
use serde_json::{de::IoRead, Deserializer};
use std::io::{Read, Write};
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

struct ServiceHandler;

impl Service for ServiceHandler {
    type Data = Yozuk;
    fn handle(&self, request: &Request, zuk: &Self::Data) -> json_rpc2::Result<Option<Response>> {
        let response = match request.method() {
            "get_commands" => {
                let req: GetCommandsRequest = request.deserialize()?;
                let tokens: Vec<Token> = req.into();
                let commands = zuk.get_commands(&tokens, &[]);
                let res = GetCommandsResponse { commands };
                Some((request, serde_json::to_value(res).unwrap()).into())
            }
            "run_commands" => {
                let req: RunCommandsRequest = request.deserialize()?;
                let result: RunCommandsResponse = zuk
                    .run_commands(req.commands, &mut [], Some(&req.i18n))
                    .into();
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
    let service: Box<dyn Service<Data = Yozuk>> = Box::new(ServiceHandler);
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

#[derive(Deserialize)]
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
            .unwrap_or_else(|| Yozuk::parse_tokens(&req.input))
    }
}

#[derive(Serialize)]
pub struct GetCommandsResponse {
    pub commands: Vec<CommandArgs>,
}

#[derive(Deserialize)]
pub struct RunCommandsRequest {
    pub commands: Vec<CommandArgs>,
    #[serde(default)]
    pub i18n: I18n,
}

#[derive(Serialize)]
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
