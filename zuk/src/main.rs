#![forbid(unsafe_code)]
#![deny(clippy::all)]

use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io;
use yozuk::Yozuk;
use yozuk_sdk::prelude::*;

mod args;
mod printer;
mod repl;
mod rpc;

use args::*;
use printer::*;

fn main() -> Result<()> {
    enter_secure_context()?;

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
        let zuk = Yozuk::builder().build();
        Ok(Self { args, zuk })
    }

    fn run(mut self) -> Result<()> {
        #[cfg(all(feature = "rpc", not(target_arch = "wasm32")))]
        if self.args.rpc {
            let stdin = io::stdin();
            let stdout = io::stdout();
            let stdin = stdin.lock();
            let stdout = stdout.lock();
            return rpc::start_server(self.zuk, stdin, stdout);
        }

        #[cfg(debug_assertions)]
        if let Some(dump_dst) = self.args.dump_model {
            use std::io::Write;
            let mut out = File::create(dump_dst)?;
            out.write_all(yozuk::MODEL_DATA)?;
            return Ok(());
        }

        let mut streams = vec![];
        if !yozuk_helper_platform::term::is_stdin_tty() {
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

            #[cfg(not(target_arch = "wasm32"))]
            {
                println!("Hi. I'm Yozuk. How may I assist you?");

                let mut repl = repl::Repl::new();
                while let Some(line) = repl.readline() {
                    let tokens = Tokenizer::new().tokenize(&line);
                    if !tokens.is_empty() {
                        self.exec_command(&tokens, &mut [])?;
                    }
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                self.exec_command(&[], &mut [])?;
            }

            Ok(())
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
}

#[cfg(all(target_os = "linux", feature = "secure-context"))]
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
                .allow_open_readonly()
                .allow_close()
                .allow_read()
                .allow_stdout()
                .allow_stderr()
                .allow_ioctl(),
        )?
        .enable(CustomRules)?
        .apply_to_all_threads()?;
    Ok(())
}

#[cfg(not(all(target_os = "linux", feature = "secure-context")))]
fn enter_secure_context() -> Result<()> {
    Ok(())
}
