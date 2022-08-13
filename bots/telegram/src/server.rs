use crate::message;
use anyhow::{bail, Result};
use futures::future::try_join_all;
use mediatype::media_type;
use reqwest::{StatusCode, Url};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use teloxide::net::Download;
use teloxide::{
    dispatching::{
        stop_token::AsyncStopToken,
        update_listeners::{self, StatefulListener},
    },
    prelude::*,
    types::{MediaKind, MessageKind, ParseMode, Update},
};
use tempfile::NamedTempFile;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

const MAX_FILE_SIZE: usize = 10485760;

pub struct Server {}

impl Server {
    pub async fn start(yozuk: Yozuk, bot: AutoSend<Bot>, endpoint: Option<Url>) -> Result<()> {
        let yozuk = Arc::new(yozuk);

        let handler = move |msg: Message, bot: AutoSend<Bot>| {
            let zuk = yozuk.clone();
            let tokenizer = Tokenizer::new();
            async move {
                if let MessageKind::Common(common) = &msg.kind {
                    let mut streams = get_streams_from_message(&bot, &msg).await?;
                    match &common.media_kind {
                        MediaKind::Text(text) if text.text == "/start" => {
                            send_hello(bot, msg).await?;
                        }
                        MediaKind::Text(text) => {
                            let mut merged_streams = vec![];
                            if let Some(reply) = &common.reply_to_message {
                                merged_streams
                                    .append(&mut get_streams_from_message(&bot, reply).await?);
                            }
                            merged_streams.append(&mut streams);
                            let tokens = tokenizer.tokenize(&text.text);
                            send_output(bot, msg, &zuk, tokens, merged_streams).await?;
                        }
                        MediaKind::Photo(photo) => {
                            let tokens =
                                tokenizer.tokenize(&photo.caption.clone().unwrap_or_default());
                            send_output(bot, msg, &zuk, tokens, streams).await?;
                        }
                        MediaKind::Audio(audio) => {
                            let tokens =
                                tokenizer.tokenize(&audio.caption.clone().unwrap_or_default());
                            send_output(bot, msg, &zuk, tokens, streams).await?;
                        }
                        MediaKind::Video(video) => {
                            let tokens =
                                tokenizer.tokenize(&video.caption.clone().unwrap_or_default());
                            send_output(bot, msg, &zuk, tokens, streams).await?;
                        }
                        MediaKind::Document(document) => {
                            let tokens =
                                tokenizer.tokenize(&document.caption.clone().unwrap_or_default());
                            send_output(bot, msg, &zuk, tokens, streams).await?;
                        }
                        _ => (),
                    }
                }
                respond(()).map_err(anyhow::Error::from)
            }
        };

        if let Some(endpoint) = endpoint {
            bot.set_webhook(endpoint).await?;
            teloxide::repl_with_listener(bot.clone(), handler, webhook(bot).await).await;
        } else {
            bot.delete_webhook().await?;
            teloxide::repl(bot.clone(), handler).await;
        }

        Ok(())
    }
}

async fn get_streams_from_message(
    bot: &AutoSend<Bot>,
    msg: &Message,
) -> anyhow::Result<Vec<InputStream>> {
    if let MessageKind::Common(common) = &msg.kind {
        match &common.media_kind {
            MediaKind::Photo(photo) => {
                let mut photos = photo.photo.clone();
                photos.sort_unstable_by_key(|image| image.width * image.height);
                return try_join_all(
                    photos
                        .iter()
                        .find(|image| image.width * image.height >= 100_000)
                        .or_else(|| photos.last())
                        .map(|image| file_stream(bot, msg, &image.file_id)),
                )
                .await;
            }
            MediaKind::Audio(audio) => {
                return Ok(vec![file_stream(bot, msg, &audio.audio.file_id).await?]);
            }
            MediaKind::Video(video) => {
                return Ok(vec![file_stream(bot, msg, &video.video.file_id).await?]);
            }
            MediaKind::Document(document) => {
                return Ok(vec![
                    file_stream(bot, msg, &document.document.file_id).await?,
                ]);
            }
            _ => (),
        }
    }
    Ok(vec![])
}

async fn file_stream(
    bot: &AutoSend<Bot>,
    msg: &Message,
    file_id: &str,
) -> anyhow::Result<InputStream> {
    let file = bot.get_file(file_id).send().await?;
    if file.file_size as usize > MAX_FILE_SIZE {
        bot.send_message(msg.chat.id, "Too large file input (10MiB max.)")
            .await?;
        bail!("Too large file input");
    }

    let tmpfile = NamedTempFile::new()?;
    let filepath = tmpfile.into_temp_path();
    let mut tmpfile = tokio::fs::File::create(&filepath).await?;
    bot.download_file(&file.file_path, &mut tmpfile).await?;
    Ok(InputStream::new(
        std::fs::File::open(filepath)?,
        media_type!(APPLICATION / OCTET_STREAM),
    ))
}

async fn send_hello(bot: AutoSend<Bot>, msg: Message) -> anyhow::Result<()> {
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
) -> anyhow::Result<()> {
    for stream in &mut streams {
        stream.read_header()?;
    }

    let commands = zuk.get_commands(&tokens, &streams);
    if commands.is_empty() {
        bot.send_message(msg.chat.id, "Sorry, I can't understand your request.")
            .await?;
        return Ok(());
    }

    let result = zuk.run_commands(commands, &mut streams, None);
    let outputs = match result {
        Ok(output) => output,
        Err(errors) => errors,
    };

    for output in outputs {
        message::render_output(bot.clone(), &msg, output).await?;
    }

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
