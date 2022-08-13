#![forbid(unsafe_code)]
#![deny(clippy::all)]

use anyhow::Result;
use clap::Parser;
use reqwest::Url;

use teloxide::prelude::*;
use yozuk::Yozuk;

mod message;
mod server;

/// Telegram Bot Server
///
/// API token must be specified via the TELOXIDE_TOKEN environment variable.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Set the webhook URL
    #[clap(short, long)]
    pub webhook: Option<Url>,

    /// Increase the logging verbosity
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let bot = Bot::from_env().auto_send();
    let yozuk = Yozuk::builder().build();
    server::Server::start(yozuk, bot, args.webhook).await
}
