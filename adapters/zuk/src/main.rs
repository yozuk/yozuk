#![forbid(unsafe_code)]
#![deny(clippy::all)]

use anyhow::Result;
use clap::Parser;
use console::Style;
use crossterm::tty::IsTty;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use sloggers::{
    terminal::{Destination, TerminalLoggerBuilder},
    types::Severity,
    Build,
};
use std::fs::File;
use std::io;
use std::io::Read;
use yozuk::{ModelSet, Yozuk, YozukError};
use yozuk_sdk::prelude::*;

mod args;
mod json;
mod printer;
mod server;

use args::*;
use printer::*;

fn main() -> Result<()> {
    let args = Args::parse();
    let app = App::new(args)?;
    app.run()
}

struct App {
    args: Args,
    zuk: Yozuk,
}

impl App {
    fn new(args: Args) -> Result<Self> {
        let config: Config = if let Some(config) = &args.config {
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

        let level = args.verbose.min(levels.len() - 1);
        let mut builder = TerminalLoggerBuilder::new();
        builder.level(levels[level]);
        builder.destination(Destination::Stderr);
        let logger = builder.build().unwrap();

        let zuk = Yozuk::builder()
            .config(config)
            .logger(logger)
            .build(ModelSet::from_data(yozuk_bundle::MODEL_DATA)?);

        Ok(Self { args, zuk })
    }

    fn run(self) -> Result<()> {
        #[cfg(feature = "server")]
        if let Some(addr) = self.args.server {
            return server::start(addr, self.args.cors_origin, self.zuk);
        }

        let stdin = io::stdin();
        let mut streams = vec![];
        if !stdin.is_tty() {
            streams.push(InputStream::new(io::stdin())?);
        }
        for file in &self.args.input {
            streams.push(InputStream::new(File::open(file)?)?);
        }

        if streams.is_empty()
            && self.args.output == OutputFormat::Term
            && self.args.query.is_empty()
        {
            self.start_repl()
        } else {
            let tokens = self
                .args
                .query
                .iter()
                .map(|token| tk!(token.clone()))
                .collect::<Vec<_>>();

            self.exec_command(&tokens, &mut streams)
        }
    }

    fn exec_command(&self, tokens: &[Token], streams: &mut [InputStream]) -> Result<()> {
        let printer = TerminalPrinter::new(&self.args);

        let commands = if self.args.run {
            Ok(vec![CommandArgs::new().add_args_iter(&self.args.query)])
        } else {
            self.zuk.get_commands(tokens, streams)
        };

        match commands {
            Ok(commands) => {
                if self.args.dry_run {
                    printer.print_commands(&commands)?;
                    return Ok(());
                }

                let result = self.zuk.run_commands(commands, streams);

                match result {
                    Ok(output) => printer.print_result(&output)?,
                    Err(YozukError::CommandError { errors }) => {
                        printer.print_error(&errors)?;
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

    fn start_repl(&self) -> Result<()> {
        println!("Hi. I'm Yozuk. How may I assist you?");
        let mut rl = Editor::<()>::new();

        let style = if console::colors_enabled() {
            Style::new().bold().blue()
        } else {
            Style::new()
        };

        loop {
            let readline = rl.readline(&format!("{} ", style.apply_to(">>")));
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());

                    let tokens = shell_words::split(&line)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|token| tk!(token))
                        .collect::<Vec<_>>();

                    if !tokens.is_empty() {
                        self.exec_command(&tokens, &mut [])?;
                    }
                }
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                    println!("Bye.");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
            }
        }

        Ok(())
    }
}
