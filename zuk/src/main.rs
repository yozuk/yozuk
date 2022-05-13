#![forbid(unsafe_code)]
#![deny(clippy::all)]

use anyhow::Result;
use clap::Parser;
use crossterm::tty::IsTty;
use owo_colors::OwoColorize;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{Context, Editor};
use rustyline_derive::Helper;
use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::Read;
use sys_locale::get_locale;
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

mod args;
mod printer;
mod rpc;

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

        let i18n = I18n {
            locale: get_locale(),
            timezone: localzone::get_local_zone(),
            ..Default::default()
        };

        let zuk = Yozuk::builder().set_config(config).set_i18n(i18n).build();
        Ok(Self { args, zuk })
    }

    fn run(mut self) -> Result<()> {
        #[cfg(feature = "rpc")]
        if self.args.rpc {
            let stdin = io::stdin();
            let stdout = io::stdout();
            let stdin = stdin.lock();
            let stdout = stdout.lock();
            return rpc::start_server(self.zuk, stdin, stdout);
        }

        enter_secure_context()?;

        let mut streams = vec![];
        if !io::stdin().is_tty() {
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

        let repl = streams.is_empty() && self.args.query.is_empty();
        if repl {
            self.args.verbose += 1;
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
            vec![CommandArgs::new().add_args_iter(&self.args.query)]
        } else {
            self.zuk.get_commands(tokens, streams)
        };

        if commands.is_empty() {
            printer.print_error_str("Sorry, I can't understand your request.")?;
            if let Some(suggest) = self.zuk.suggest(tokens) {
                printer.print_suggest(&suggest)?;
            }
        } else {
            if self.args.dry_run {
                printer.print_commands(&commands)?;
                return Ok(());
            }

            let result = self.zuk.run_commands(commands, streams, None);

            match result {
                Ok(outputs) => {
                    for i in 0..outputs.len() {
                        printer.print_result(&outputs[i])?;
                        if outputs.len() > 1 && i < outputs.len() - 1 {
                            println!("--");
                        }
                    }
                }
                Err(errors) => {
                    printer.print_error(&errors)?;
                }
            }
        }

        Ok(())
    }

    fn start_repl(&self) -> Result<()> {
        let mut rl = Editor::new();

        let prompt = "» ";
        let helper = YozukHelper {
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            colored_prompt: "".to_owned(),
            validator: MatchingBracketValidator::new(),
        };
        rl.set_helper(Some(helper));
        rl.helper_mut().expect("No helper").colored_prompt = format!("{}", prompt.bold().blue());

        println!("Hi. I'm Yozuk. How may I assist you?");
        loop {
            let readline = rl.readline(prompt);
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());

                    let tokens = Tokenizer::new().tokenize(&line);
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

#[cfg(target_os = "linux")]
fn enter_secure_context() -> Result<()> {
    use extrasafe::builtins::{danger_zone::Threads, SystemIO};
    use extrasafe::{Rule, RuleSet, SafetyContext};
    use std::collections::HashMap;
    use syscalls::Sysno;

    struct CustomRules;

    impl RuleSet for CustomRules {
        fn simple_rules(&self) -> Vec<Sysno> {
            vec![Sysno::poll, Sysno::ppoll]
        }

        fn conditional_rules(&self) -> HashMap<Sysno, Vec<Rule>> {
            HashMap::new()
        }

        fn name(&self) -> &'static str {
            "CustomRules"
        }
    }

    SafetyContext::new()
        .enable(Threads::nothing().allow_create())?
        .enable(
            SystemIO::nothing()
                .allow_stdin()
                .allow_stdout()
                .allow_stderr()
                .allow_ioctl(),
        )?
        .enable(CustomRules)?
        .apply_to_all_threads()?;
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn enter_secure_context() -> Result<()> {
    Ok(())
}
