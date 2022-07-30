#![forbid(unsafe_code)]
#![deny(clippy::all)]

use anyhow::Result;
use clap::Parser;
use reqwest::Url;
use slog::info;
use sloggers::{
    terminal::{Destination, TerminalLoggerBuilder},
    types::Severity,
    Build,
};

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
    let levels = [Severity::Info, Severity::Debug, Severity::Trace];

    let level = args.verbose.min(levels.len() - 1);
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(levels[level]);
    builder.destination(Destination::Stderr);
    let logger = builder.build().unwrap();

    info!(logger, "Starting yozuk-telegram...");

    let bot = Bot::from_env().auto_send();

    if let Some(webhook) = args.webhook {
        bot.set_webhook(webhook)
            .await
            .expect("Cannot setup a webhook");
    }

    let yozuk = Yozuk::builder().build();

    server::Server::start(yozuk, logger, bot).await;
    Ok(())
}
