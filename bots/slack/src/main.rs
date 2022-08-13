use anyhow::Result;
use clap::Parser;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use lazy_regex::regex_replace_all;
use mediatype::MediaTypeBuf;
use reqwest::{header, multipart, Url};
use serde_derive::Deserialize;
use std::convert::Infallible;
use std::net::SocketAddrV4;
use std::str;
use std::str::FromStr;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt;
use warp::Filter;
use websocket_lite::Opcode;
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

mod args;
mod block;
mod event;
mod message;
mod user;

use args::*;
use block::*;
use event::*;
use message::*;
use user::*;

const API_URL_AUTH_TEST: &str = "https://slack.com/api/auth.test";
const API_URL_POST_MESSAGE: &str = "https://slack.com/api/chat.postMessage";
const API_URL_VIEWS_PUBLISH: &str = "https://slack.com/api/views.publish";
const API_URL_USERS_INFO: &str = "https://slack.com/api/users.info";
const API_URL_FILES_UPLOAD: &str = "https://slack.com/api/files.upload";
const API_URL_CONNECTIONS_OPEN: &str = "https://slack.com/api/apps.connections.open";

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::try_parse()?;
    let yozuk = Arc::new(Yozuk::builder().build());

    let mut headers = header::HeaderMap::new();
    let mut auth_value =
        header::HeaderValue::from_str(&format!("Bearer {}", args.bot_token)).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let bot_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let identity = bot_client
        .post(API_URL_AUTH_TEST)
        .send()
        .await?
        .json::<Identity>()
        .await?;

    if let Some(app_token) = &args.app_token {
        let mut headers = header::HeaderMap::new();
        let mut auth_value =
            header::HeaderValue::from_str(&format!("Bearer {}", app_token)).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let app_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let connection = app_client
            .post(API_URL_CONNECTIONS_OPEN)
            .send()
            .await?
            .json::<Connection>()
            .await?;

        let builder = websocket_lite::ClientBuilder::new(connection.url.as_str())?;
        let mut ws_stream = builder.async_connect().await.unwrap();

        while let Some(msg) = ws_stream.next().await {
            let msg = if let Ok(msg) = msg {
                msg
            } else {
                let _ = ws_stream.send(websocket_lite::Message::close(None)).await;
                break;
            };

            match msg.opcode() {
                Opcode::Text => {
                    let envelope: Result<Envelope, _> =
                        serde_json::from_str(msg.as_text().unwrap());
                    if let Ok(envelope) = envelope {
                        ws_stream
                            .send(websocket_lite::Message::text(
                                serde_json::to_string(&EnvelopeAck {
                                    envelope_id: envelope.envelope_id,
                                })
                                .unwrap(),
                            ))
                            .await
                            .unwrap();
                        let EnvelopeData::EventsApi(api) = envelope.data;
                        handle_message(
                            api.payload,
                            yozuk.clone(),
                            bot_client.clone(),
                            identity.clone(),
                        )
                        .await?;
                    }
                }
                Opcode::Binary => {
                    println!("{:?}", msg);
                }
                Opcode::Ping => ws_stream
                    .send(websocket_lite::Message::pong(msg.into_data()))
                    .await
                    .unwrap(),
                Opcode::Close => {
                    let _ = ws_stream.send(websocket_lite::Message::close(None)).await;
                    break;
                }
                Opcode::Pong => {}
            }
        }
    } else {
        let route = warp::any().and(warp::body::json()).and_then(move |event| {
            handle_message(event, yozuk.clone(), bot_client.clone(), identity.clone())
        });

        warp::serve(route)
            .run(SocketAddrV4::new(args.addr, args.port))
            .await;
    }

    Ok(())
}

async fn handle_message(
    event: Event,
    zuk: Arc<Yozuk>,
    client: reqwest::Client,
    identity: Identity,
) -> Result<warp::reply::Json, Infallible> {
    match event {
        Event::EventCallback(cb) => match cb.event {
            MessageEvent::AppMention(msg) => {
                handle_request(msg, zuk, client).await.unwrap();
            }
            MessageEvent::Message(msg) => {
                if msg.user != identity.user_id {
                    handle_request(msg, zuk, client).await.unwrap();
                }
            }
            MessageEvent::AppHomeOpened(event) => {
                publish_home(client, event.user).await.unwrap();
            }
        },
        Event::UrlVerification(event) => return Ok(handle_url_verification(event)),
    }
    Ok(warp::reply::json(&"ok".to_string()))
}

async fn publish_home(client: reqwest::Client, user_id: String) -> Result<()> {
    client
        .post(API_URL_VIEWS_PUBLISH)
        .json(&ViewsPublish {
            user_id,
            view: View {
                ty: "home".into(),
                blocks: vec![SlackBlock {
                    ty: "section".into(),
                    text: Some(Text {
                        ty: "mrkdwn".into(),
                        text: "Hello, I'm Yozuk.".into(),
                    }),
                }],
            },
        })
        .send()
        .await?;
    Ok(())
}

async fn handle_request(msg: Message, zuk: Arc<Yozuk>, client: reqwest::Client) -> Result<()> {
    let user = client
        .get(API_URL_USERS_INFO)
        .query(&[("user", msg.user.as_str()), ("include_locale", "true")])
        .send()
        .await?
        .json::<UserResponse>()
        .await
        .map(|res| res.user)
        .unwrap_or_else(|_| User {
            id: msg.user.clone(),
            ..Default::default()
        });

    let text = regex_replace_all!(
        r#"<@\w+>"#i,
        & msg.text,
        |_| String::new(),
    );
    let text = regex_replace_all!(
        r#"<[^|]+\|([^>]+)>"#i,
        &text,
        |_, text: &str| text.to_string(),
    );
    let text = regex_replace_all!(
        r#"<([^>]+)>"#i,
        &text,
        |_, text: &str| text.to_string(),
    );
    let text = gh_emoji::Replacer::new().replace_all(&text);

    let mut streams = futures::future::try_join_all(msg.files.iter().map(file_stream)).await?;

    let tokens = Tokenizer::new().tokenize(&text);
    let user = UserContext {
        timezone: user.tz,
        ..Default::default()
    };

    let commands = zuk.get_commands(&tokens, &streams);
    if commands.is_empty() {
        let massage = PostMessage {
            channel: msg.channel.clone(),
            text: Some("Sorry, I can't understand your request.".into()),
            ..Default::default()
        };
        client
            .post(API_URL_POST_MESSAGE)
            .json(&massage)
            .send()
            .await?;
        return Ok(());
    }

    let result = zuk.run_commands(commands, &mut streams, Some(&user));
    let outputs = match result {
        Ok(outputs) => outputs,
        Err(outputs) => outputs,
    };

    for output in outputs {
        for block in output.blocks {
            let message = match block {
                Block::Comment(comment) => PostMessage {
                    channel: msg.channel.clone(),
                    text: Some(comment.text),
                    ..Default::default()
                },
                Block::Data(data) => {
                    if let Ok(text) = str::from_utf8(&data.data) {
                        PostMessage {
                            channel: msg.channel.clone(),
                            blocks: Some(vec![SlackBlock {
                                ty: "section".into(),
                                text: Some(Text {
                                    ty: "mrkdwn".into(),
                                    text: text.into(),
                                }),
                            }]),
                            ..Default::default()
                        }
                    } else {
                        let file = multipart::Part::bytes(data.data.to_vec())
                            .file_name(data.file_name)
                            .mime_str(&data.media_type.to_string())?;
                        let form = multipart::Form::new()
                            .part("file", file)
                            .text("channels", msg.channel.clone());
                        client
                            .post(API_URL_FILES_UPLOAD)
                            .multipart(form)
                            .send()
                            .await?;
                        continue;
                    }
                }
                _ => continue,
            };
            client
                .post(API_URL_POST_MESSAGE)
                .json(&message)
                .send()
                .await?;
        }
    }

    Ok(())
}

fn handle_url_verification(verification: UrlVerification) -> warp::reply::Json {
    warp::reply::json(&UrlVerificationReply {
        challenge: verification.challenge,
    })
}

async fn file_stream(file: &File) -> anyhow::Result<InputStream> {
    let tmpfile = NamedTempFile::new()?;
    let filepath = tmpfile.into_temp_path();
    let mut tmpfile = tokio::fs::File::create(&filepath).await?;
    let mut stream = reqwest::get(&file.url_private_download)
        .await?
        .bytes_stream();
    while let Some(data) = stream.next().await {
        let _ = tmpfile.write(&data?).await?;
    }
    Ok(InputStream::new(
        std::fs::File::open(filepath)?,
        MediaTypeBuf::from_str(&file.mimetype).unwrap(),
    ))
}

#[derive(Debug, Clone, Deserialize)]
pub struct Connection {
    pub url: Url,
}
