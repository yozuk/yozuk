use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct View {
    #[serde(rename = "type")]
    pub ty: String,

    pub blocks: Vec<SlackBlock>,
}

#[derive(Debug, Serialize)]
pub struct SlackBlock {
    #[serde(rename = "type")]
    pub ty: String,

    pub text: Option<Text>,
}

#[derive(Debug, Serialize)]
pub struct Text {
    #[serde(rename = "type")]
    pub ty: String,

    pub text: String,
}
