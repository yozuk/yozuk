use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    UrlVerification(UrlVerification),
    EventCallback(EventCallback),
}

#[derive(Debug, Deserialize)]
pub struct UrlVerification {
    pub token: String,
    pub challenge: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageEvent {
    AppMention(Message),
    Message(Message),
    AppHomeOpened(AppHomeOpened),
}

#[derive(Debug, Deserialize)]
pub struct EventCallback {
    pub event: MessageEvent,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub channel: String,
    pub text: String,
    pub user: String,
    pub ts: String,
    #[serde(default)]
    pub files: Vec<File>,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub name: String,
    pub title: String,
    pub mimetype: String,
    pub url_private_download: String,
}

#[derive(Debug, Deserialize)]
pub struct AppHomeOpened {
    pub user: String,
}

#[derive(Debug, Deserialize)]
pub struct Envelope {
    pub envelope_id: String,

    #[serde(flatten)]
    pub data: EnvelopeData,
}

#[derive(Debug, Serialize)]
pub struct EnvelopeAck {
    pub envelope_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EnvelopeData {
    EventsApi(EventsApi),
}

#[derive(Debug, Deserialize)]
pub struct EventsApi {
    pub payload: Event,
}
