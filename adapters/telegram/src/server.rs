use crate::message;
use futures::future::try_join_all;
use reqwest::StatusCode;
use slog::Logger;
use slog::{debug, error};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use teloxide::net::Download;
use teloxide::DownloadError;
use teloxide::{
    dispatching::{
        stop_token::AsyncStopToken,
        update_listeners::{self, StatefulListener},
    },
    prelude2::*,
    types::{MediaKind, MessageKind, ParseMode, Update},
    RequestError,
};
use tempfile::NamedTempFile;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;
use yozuk::{Yozuk, YozukError};
use yozuk_sdk::prelude::*;

pub struct Server {}

impl Server {
    pub async fn start(yozuk: Yozuk, logger: Logger, bot: AutoSend<Bot>) {
        let yozuk = Arc::new(yozuk);

        teloxide::repls2::repl_with_listener(
            bot.clone(),
            move |msg: Message, bot: AutoSend<Bot>| {
                debug!(logger, "recv: {:?}", msg);

                let zuk = yozuk.clone();
                let logger = logger.clone();
                async move {
                    if let MessageKind::Common(common) = &msg.kind {
                        match &common.media_kind {
                            MediaKind::Text(text) if text.text == "/start" => {
                                send_hello(bot, msg).await?;
                            }
                            MediaKind::Text(text) => {
                                let words = shell_words::split(&text.text).ok().unwrap_or_default();
                                let tokens = words
                                    .into_iter()
                                    .map(|token| tk!(token))
                                    .collect::<Vec<_>>();
                                if let Err(err) =
                                    send_output(bot, msg, &zuk, tokens, vec![], logger.clone())
                                        .await
                                {
                                    error!(logger, "{}", err);
                                }
                            }
                            MediaKind::Photo(photo) => {
                                let mut photos = photo.photo.clone();
                                photos.sort_unstable_by_key(|image| image.width * image.height);
                                let paths = try_join_all(
                                    photos
                                        .iter()
                                        .find(|image| image.width * image.height >= 100_000)
                                        .or_else(|| photos.last())
                                        .map(|image| bot.get_file(image.file_id.clone()).send()),
                                )
                                .await?;
                                let files = try_join_all(
                                    paths
                                        .into_iter()
                                        .map(|file| download_file(&bot, file.file_path)),
                                )
                                .await
                                .unwrap();
                                let streams = files
                                    .into_iter()
                                    .filter_map(|file| InputStream::new(file).ok())
                                    .collect();
                                let words =
                                    shell_words::split(&photo.caption.clone().unwrap_or_default())
                                        .ok()
                                        .unwrap_or_default();
                                let tokens = words.into_iter().map(|token| tk!(token)).collect();
                                if let Err(err) =
                                    send_output(bot, msg, &zuk, tokens, streams, logger.clone())
                                        .await
                                {
                                    error!(logger, "{}", err);
                                }
                            }
                            _ => (),
                        }
                    }
                    respond(())
                }
            },
            webhook(bot).await,
        )
        .await;
    }
}

async fn download_file(bot: &AutoSend<Bot>, path: String) -> Result<std::fs::File, DownloadError> {
    let tmpfile = NamedTempFile::new().unwrap();
    let filepath = tmpfile.into_temp_path();
    let mut tmpfile = tokio::fs::File::create(&filepath).await.unwrap();
    bot.download_file(&path, &mut tmpfile).await?;
    Ok(std::fs::File::open(filepath).unwrap())
}

async fn send_hello(bot: AutoSend<Bot>, msg: Message) -> Result<(), RequestError> {
    bot.send_message(
        msg.chat.id,
        "Hi. I'm <b>Yozuk</b>.\nHow may I assist you?".to_string(),
    )
    .parse_mode(ParseMode::Html)
    .send()
    .await?;

    Ok(())
}

async fn send_output(
    bot: AutoSend<Bot>,
    msg: Message,
    zuk: &Yozuk,
    tokens: Vec<Token>,
    mut streams: Vec<InputStream>,
    logger: Logger,
) -> Result<(), RequestError> {
    let result = zuk
        .get_commands(&tokens, &[])
        .and_then(|commands| zuk.run_commands(commands, &mut streams));

    debug!(logger, "result: {:?}", result);

    let output = match result {
        Ok(output) => output,
        Err(YozukError::UnintelligibleRequest { suggest }) => {
            bot.send_message(msg.chat.id, "Sorry, I can't understand your request.")
                .await?;
            if let Some(suggest) = suggest {
                bot.send_message(
                    msg.chat.id,
                    format!("💡Did you mean <em>{}</em> ?", suggest),
                )
                .parse_mode(ParseMode::Html)
                .send()
                .await?;
            }
            return Ok(());
        }
        Err(YozukError::CommandError { mut errors }) => errors.pop().unwrap(),
    };

    message::render_output(bot, &msg, output).await?;

    Ok(())
}

async fn handle_rejection(error: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    log::error!("Cannot process the request due to: {:?}", error);
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

async fn webhook(_bot: AutoSend<Bot>) -> impl update_listeners::UpdateListener<Infallible> {
    let (tx, rx) = mpsc::unbounded_channel();

    let server = warp::post()
        .and(warp::body::json())
        .map(move |update: Update| {
            tx.send(Ok(update))
                .expect("Cannot send an incoming update from the webhook");

            StatusCode::OK
        })
        .recover(handle_rejection);

    let (stop_token, stop_flag) = AsyncStopToken::new_pair();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();
    let server = warp::serve(server);
    let (_addr, fut) = server.bind_with_graceful_shutdown(addr, stop_flag);

    tokio::spawn(fut);
    let stream = UnboundedReceiverStream::new(rx);

    fn streamf<S, T>(state: &mut (S, T)) -> &mut S {
        &mut state.0
    }
    StatefulListener::new(
        (stream, stop_token),
        streamf,
        |state: &mut (_, AsyncStopToken)| state.1.clone(),
    )
}
