#![cfg(feature = "rpc")]

use anyhow::Result;
use json_rpc2::{Request, Response, Server, Service};
use serde_json::{de::IoRead, Deserializer, Value};
use std::io::{Read, Write};

struct ServiceHandler;
impl Service for ServiceHandler {
    type Data = ();
    fn handle(&self, request: &Request, _ctx: &Self::Data) -> json_rpc2::Result<Option<Response>> {
        let response = match request.method() {
            "hello" => {
                let params: String = request.deserialize()?;
                let message = format!("Hello, {}!", params);
                Some((request, Value::String(message)).into())
            }
            _ => None,
        };
        Ok(response)
    }
}

pub fn start_server<R, W>(reader: R, mut writer: W) -> Result<()>
where
    R: Read,
    W: Write,
{
    let service: Box<dyn Service<Data = ()>> = Box::new(ServiceHandler {});
    let server = Server::new(vec![&service]);
    let reader = IoRead::new(reader);
    let stream = Deserializer::new(reader).into_iter::<Request>();
    for request in stream {
        let response = server.serve(&request?, &());
        serde_json::to_writer(&mut writer, &response)?;
        writeln!(&mut writer)?;
    }
    Ok(())
}
