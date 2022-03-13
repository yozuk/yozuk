#![forbid(unsafe_code)]

use anyhow::Result;
use clap::Parser;
use reqwest::Url;
use slog::info;
use sloggers::{
    terminal::{Destination, TerminalLoggerBuilder},
    types::Severity,
    Build,
};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use teloxide::prelude2::*;
use yozuk::{ModelSet, Yozuk};
use yozuk_sdk::prelude::*;

mod message;
mod server;

/// Telegram Bot Server
///
/// API token must be specified via the TELOXIDE_TOKEN environment variable.
#[derive(Debug, Parser)]
pub struct Args {
    /// Set the webhook URL
    #[clap(short, long)]
    pub webhook: Option<Url>,

    /// Increase the logging verbosity
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,

    /// Load config from a TOML file
    #[clap(short, long)]
    pub config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config: Config = if let Some(config) = &args.config {
        let mut file = File::open(config)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        toml::from_slice(&data)?
    } else {
        Default::default()
    };

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

    let model = ModelSet::from_data(yozuk_bundle::MODEL_DATA).unwrap();
    let yozuk = Yozuk::builder()
        .logger(logger.clone())
        .config(config)
        .build(model);

    server::Server::start(yozuk, logger, bot).await;
    Ok(())
}
