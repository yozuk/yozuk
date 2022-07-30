use super::block::*;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct UrlVerificationReply {
    pub challenge: String,
}

#[derive(Debug, Default, Serialize)]
pub struct PostMessage {
    pub channel: String,
    pub text: Option<String>,
    pub blocks: Option<Vec<SlackBlock>>,
    pub thread_ts: Option<String>,
    pub reply_broadcast: bool,
}

#[derive(Debug, Default, Serialize)]
pub struct PostEphemeral {
    pub channel: String,
    pub text: String,
    pub user: String,
}

#[derive(Debug, Serialize)]
pub struct ViewsPublish {
    pub user_id: String,
    pub view: View,
}
