#![cfg(feature = "rpc")]

use json_rpc2::*;
use serde_json::Value;

struct ServiceHandler;
impl Service for ServiceHandler {
    type Data = ();
    fn handle(&self, request: &Request, _ctx: &Self::Data) -> Result<Option<Response>> {
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

fn start() -> Result<()> {
    let service: Box<dyn Service<Data = ()>> = Box::new(ServiceHandler {});
    let request = Request::new_reply("hello", Some(Value::String("world".to_string())));
    let server = Server::new(vec![&service]);
    let response = server.serve(&request, &());
    assert_eq!(
        Some(Value::String("Hello, world!".to_string())),
        response.unwrap().into()
    );
    Ok(())
}
