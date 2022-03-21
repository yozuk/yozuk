#![cfg(feature = "server")]

use super::json::{JsonInput, JsonResult};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::runtime::Runtime;
use warp::http::StatusCode;
use warp::reply::WithStatus;
use warp::Filter;
use yozuk::{Yozuk, YozukError};
use yozuk_sdk::prelude::*;

pub fn start(addr: SocketAddr, allow_origins: Vec<String>, zuk: Yozuk) -> anyhow::Result<()> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let zuk = Arc::new(zuk);

        let mut cors = warp::cors()
            .allow_methods(vec!["POST"])
            .allow_headers(vec!["content-type"]);

        for origin in allow_origins {
            cors = cors.allow_origin(origin.as_str());
        }

        let run = warp::post()
            .and(warp::path("run"))
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(move |input: JsonInput| run_command(&zuk, &input.tokens))
            .with(cors);

        warp::serve(run).run(addr).await;
        Ok(())
    })
}

fn run_command(zuk: &Yozuk, tokens: &[Token]) -> WithStatus<warp::reply::Json> {
    let commands = match zuk.get_commands(tokens, &[]) {
        Ok(commands) => commands,
        Err(err) => {
            return warp::reply::with_status(
                warp::reply::json(&JsonResult::Error {
                    message: &err.to_string(),
                }),
                StatusCode::BAD_REQUEST,
            );
        }
    };
    match zuk.run_commands(commands, &mut []) {
        Ok(output) => warp::reply::with_status(
            warp::reply::json(&JsonResult::Ok { output: &output }),
            StatusCode::OK,
        ),
        Err(YozukError::CommandError { errors }) => warp::reply::with_status(
            warp::reply::json(&JsonResult::Fail { outputs: &errors }),
            StatusCode::BAD_REQUEST,
        ),
        Err(err) => warp::reply::with_status(
            warp::reply::json(&JsonResult::Error {
                message: &err.to_string(),
            }),
            StatusCode::BAD_REQUEST,
        ),
    }
}
