#![forbid(unsafe_code)]
#![deny(clippy::all)]

use anyhow::Result;
use clap::Parser;
use sloggers::{
    terminal::{Destination, TerminalLoggerBuilder},
    types::Severity,
    Build,
};
use std::fs::File;
use std::io::Read;
use yozuk::{ModelSet, Yozuk, YozukError};
use yozuk_sdk::prelude::*;

mod args;
mod printer;

use args::*;
use printer::*;

fn main() -> Result<()> {
    let args = Args::parse();
    let app = App::new(args)?;
    app.run()
}

struct App {
    args: Args,
}

impl App {
    fn new(args: Args) -> Result<Self> {
        Ok(Self { args })
    }

    fn run(&self) -> Result<()> {
        let config: Config = if let Some(config) = &self.args.config {
            let mut file = File::open(config)?;
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            toml::from_slice(&data)?
        } else {
            Default::default()
        };

        let levels = [
            Severity::Error,
            Severity::Info,
            Severity::Debug,
            Severity::Trace,
        ];
        let level = self.args.verbose.min(levels.len() - 1);
        let mut builder = TerminalLoggerBuilder::new();
        builder.level(levels[level]);
        builder.destination(Destination::Stderr);
        let logger = builder.build().unwrap();

        let zuk = Yozuk::builder()
            .config(config)
            .logger(logger)
            .build(ModelSet::from_data(yozuk_bundle::MODEL_DATA)?);

        let tokens = self
            .args
            .query
            .iter()
            .map(|token| tk!(token.clone()))
            .collect::<Vec<_>>();

        let printer = TerminalPrinter::new(&self.args);

        let commands = if self.args.run {
            Ok(vec![CommandArgs::new().add_args_iter(&self.args.query)])
        } else {
            zuk.get_commands(&tokens)
        };

        match commands {
            Ok(commands) => {
                if self.args.dry_run {
                    printer.print_commands(&commands)?;
                    return Ok(());
                }

                let result = zuk.run_commands(commands);

                match result {
                    Ok(output) => printer.print_result(&output)?,
                    Err(YozukError::CommandError { errors }) => {
                        printer.print_error(&errors[0])?;
                    }
                    _ => (),
                }
            }
            Err(YozukError::UnintelligibleRequest { suggest }) => {
                printer.print_error_str("Sorry, I can't understand your request.")?;
                if let Some(suggest) = suggest {
                    printer.print_suggest(&suggest)?;
                }
            }
            _ => (),
        }

        Ok(())
    }
}
