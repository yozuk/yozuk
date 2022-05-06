#![cfg(feature = "rpc")]

use anyhow::Result;
use json_rpc2::{Request, Response, Server, Service};
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
                let tokens: Vec<Token> = request.deserialize()?;
                let commands = zuk.get_commands(&tokens, &[]);
                Some((request, serde_json::to_value(commands).unwrap()).into())
            }
            "run_commands" => {
                let commands: Vec<CommandArgs> = request.deserialize()?;
                let result = zuk.run_commands(commands, &mut [], None);
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
