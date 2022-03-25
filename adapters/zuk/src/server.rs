#![cfg(feature = "server")]

use super::json::{JsonInput, JsonResult};
use futures_util::StreamExt;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::runtime::Runtime;
use warp::http::StatusCode;
use warp::reply::WithStatus;
use warp::Buf;
use warp::Filter;
use yozuk::{Yozuk, YozukError};
use yozuk_sdk::prelude::*;

const CONTENT_LENGTH_LIMIT: u64 = 1024 * 1024 * 20;

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
            .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
            .and(warp::multipart::form())
            .and_then(decode_form)
            .map(move |input| run_command(&zuk, input))
            .with(cors);

        warp::serve(run).run(addr).await;
        Ok(())
    })
}

async fn decode_form(
    mut form: warp::multipart::FormData,
) -> Result<(Option<JsonInput>, Vec<InputStream>), Infallible> {
    let mut input: Option<JsonInput> = None;
    let mut streams = vec![];

    while let Some(Ok(mut part)) = form.next().await {
        if let Some(Ok(data)) = part.data().await {
            if input.is_none() && part.name() == "query.json" {
                input = serde_json::from_reader(data.reader()).ok();
            } else {
                streams.push(InputStream::new(data.reader()));
            }
        }
    }
    Ok((input, streams))
}

fn run_command(
    zuk: &Yozuk,
    (input, mut streams): (Option<JsonInput>, Vec<InputStream>),
) -> WithStatus<warp::reply::Json> {
    let input = match input {
        Some(input) => input,
        None => {
            return warp::reply::with_status(
                warp::reply::json(&JsonResult::Error {
                    message: "missing query.json",
                }),
                StatusCode::BAD_REQUEST,
            );
        }
    };
    for stream in &mut streams {
        if let Err(err) = stream.read_header() {
            return warp::reply::with_status(
                warp::reply::json(&JsonResult::Error {
                    message: &err.to_string(),
                }),
                StatusCode::BAD_REQUEST,
            );
        }
    }
    let commands = match zuk.get_commands(&input.tokens, &streams) {
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
    match zuk.run_commands(commands, &mut streams) {
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
