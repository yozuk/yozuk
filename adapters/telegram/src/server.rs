use crate::message;
use reqwest::StatusCode;
use slog::Logger;
use slog::{debug, error};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use teloxide::{
    dispatching::{
        stop_token::AsyncStopToken,
        update_listeners::{self, StatefulListener},
    },
    prelude2::*,
    types::{MediaKind, MessageKind, ParseMode, Update},
    RequestError,
};
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
                                    send_output(bot, msg, &zuk, tokens, logger.clone()).await
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
    logger: Logger,
) -> Result<(), RequestError> {
    let result = zuk
        .get_commands(&tokens, &[])
        .and_then(|commands| zuk.run_commands(commands));

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
