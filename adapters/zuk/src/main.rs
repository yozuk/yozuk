#![forbid(unsafe_code)]
#![deny(clippy::all)]

use anyhow::Result;
use clap::Parser;
use console::Style;
use crossterm::tty::IsTty;
use mediatype::media_type;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{Context, Editor};
use rustyline_derive::Helper;
use sloggers::{
    terminal::{Destination, TerminalLoggerBuilder},
    types::Severity,
    Build,
};
use std::borrow::Cow;
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
        #[cfg(feature = "http-server")]
        if let Some(addr) = self.args.server_addr {
            return server::start(addr, self.args.cors_origin, self.zuk);
        }

        let stdin = io::stdin();
        let mut streams = vec![];
        if !stdin.is_tty() {
            streams.push(InputStream::new(
                io::stdin(),
                media_type!(APPLICATION / OCTET_STREAM),
            ));
        }
        for file in &self.args.input {
            streams.push(InputStream::new(
                File::open(file)?,
                media_type!(APPLICATION / OCTET_STREAM),
            ));
        }

        let repl = (self.args.mode == Mode::Auto
            && streams.is_empty()
            && self.args.output == OutputFormat::Term
            && self.args.query.is_empty())
            || self.args.mode == Mode::Repl;

        if repl {
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
        for stream in streams.iter_mut() {
            stream.read_header()?;
        }

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
        let mut rl = Editor::new();

        let style = if console::colors_enabled() {
            Style::new().bold().blue()
        } else {
            Style::new()
        };

        let prompt = "» ";
        let helper = YozukHelper {
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            colored_prompt: "".to_owned(),
            validator: MatchingBracketValidator::new(),
        };
        rl.set_helper(Some(helper));
        rl.helper_mut().expect("No helper").colored_prompt = format!("{}", style.apply_to(&prompt));

        println!("Hi. I'm Yozuk. How may I assist you?");
        loop {
            let readline = rl.readline(prompt);
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

#[derive(Helper)]
struct YozukHelper {
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for YozukHelper {
    type Candidate = Pair;
}

impl Hinter for YozukHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for YozukHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Cow::Borrowed(&self.colored_prompt)
        } else {
            Cow::Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for YozukHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}
