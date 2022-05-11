use crate::Args;
use anyhow::Result;
use crossterm::tty::IsTty;
use hexyl::{BorderStyle, Printer};
use owo_colors::OwoColorize;
use std::io::BufRead;
use std::io::{self, Write};
use std::str;
use yozuk_sdk::prelude::*;

pub struct TerminalPrinter<'a> {
    args: &'a Args,
}

impl<'a> TerminalPrinter<'a> {
    pub fn new(args: &'a Args) -> Self {
        Self { args }
    }

    pub fn print_commands(&self, commands: &[CommandArgs]) -> Result<()> {
        let mut stdout = io::stdout();
        for cmd in commands {
            writeln!(&mut stdout, "{}", shell_words::join(&cmd.args))?;
        }
        Ok(())
    }

    pub fn print_result(&self, output: &Output) -> Result<()> {
        let mut stdout = io::stdout();
        let mut stderr = io::stderr();
        let stdin = io::stdin();

        let title = if output.title.is_empty() {
            String::new()
        } else {
            format!("{}", format!("{}: ", output.title).bold().green())
        };

        for block in &output.blocks {
            match block {
                Block::Comment(comment) => {
                    if self.args.verbose > 0 {
                        writeln!(&mut stderr, "{}{}", title, comment.text)?;
                    }
                }
                Block::Data(data) => {
                    let data = data.data.data().unwrap();
                    let printable = str::from_utf8(data).is_ok();
                    if printable {
                        stdout.write_all(data)?;
                        writeln!(&mut stdout)?;
                    } else {
                        self.print_binary(data)?;
                    }
                }
                Block::Preview(block::Preview::Color(color)) => {
                    writeln!(
                        &mut stderr,
                        "{}",
                        "       ".on_truecolor(color.red, color.green, color.blue)
                    )?;
                }
                Block::Spoiler(spoiler) => {
                    use crossterm::{
                        cursor::MoveToPreviousLine,
                        execute,
                        terminal::{Clear, ClearType},
                    };
                    write!(
                        &mut stderr,
                        "{} Press enter to show {}",
                        "Spoiler:".bold(),
                        spoiler.title.on_red()
                    )?;
                    stdin.lock().lines().next().unwrap()?;
                    execute!(
                        stderr,
                        MoveToPreviousLine(1),
                        Clear(ClearType::FromCursorDown)
                    )?;
                    writeln!(
                        &mut stderr,
                        "{}{} {}",
                        spoiler.title.on_red(),
                        ":".on_red(),
                        spoiler.data.unsecure()
                    )?;
                    write!(&mut stderr, "{}", "Press enter to hide".dimmed())?;
                    stdin.lock().lines().next().unwrap()?;
                    execute!(
                        stderr,
                        MoveToPreviousLine(2),
                        Clear(ClearType::FromCursorDown)
                    )?;
                }
                _ => {
                    if self.args.verbose > 0 {
                        writeln!(&mut stderr, "{}", "[unimplemented]".dimmed())?
                    }
                }
            }
        }

        Ok(())
    }

    pub fn print_error(&self, outputs: &[Output]) -> Result<()> {
        let mut stderr = io::stderr();

        let output = &outputs[0];
        let title = if output.title.is_empty() {
            String::new()
        } else {
            format!("{}", format!("{}: ", output.title).bold().red())
        };

        for block in &output.blocks {
            if let Block::Comment(comment) = block {
                writeln!(&mut stderr, "{}{}", title, comment.text)?;
            } else {
                writeln!(&mut stderr, "{}", "[unimplemented]".dimmed())?;
            }
        }

        Ok(())
    }

    pub fn print_error_str(&self, err: &str) -> Result<()> {
        let mut stderr = io::stderr();
        writeln!(&mut stderr, "{}", err.red())?;
        Ok(())
    }

    pub fn print_suggest(&self, suggest: &str) -> Result<()> {
        let mut stderr = io::stderr();
        writeln!(&mut stderr, "Did you mean {} ?", suggest.italic())?;
        Ok(())
    }

    fn print_binary(&self, data: &[u8]) -> Result<()> {
        let mut stdout = io::stdout();
        if stdout.is_tty() {
            let show_color = true;
            let use_squeezing = false;
            let border_style = BorderStyle::Unicode;
            let mut printer = Printer::new(&mut stdout, show_color, border_style, use_squeezing);
            printer.print_all(data).unwrap();
        } else {
            stdout.write_all(data)?;
        }
        Ok(())
    }
}
