use crate::Args;
use anyhow::Result;
use chardetng::EncodingDetector;
use hexyl::{BorderStyle, Printer};
use owo_colors::OwoColorize;
use std::io::{self, Write};
use std::str;
use std::str::FromStr;
use yozuk_sdk::prelude::*;

pub struct TerminalPrinter<'a> {
    args: &'a Args,
}

impl<'a> TerminalPrinter<'a> {
    pub fn new(args: &'a Args) -> Self {
        Self { args }
    }

    pub fn print_commands(&self, commands: &[CommandArgs]) -> Result<()> {
        self.print_json(&commands, io::stderr().lock())?;

        let mut stdout = io::stdout();
        for cmd in commands {
            writeln!(&mut stdout, "{}", shell_words::join(&cmd.args))?;
        }
        Ok(())
    }

    pub fn print_json<T, W>(&self, data: &T, mut output: W) -> Result<()>
    where
        T: serde::Serialize,
        W: Write,
    {
        if self.args.verbose > 1 {
            writeln!(
                &mut output,
                "{}",
                serde_json::to_string_pretty(&data).unwrap().dimmed()
            )?;
        }
        Ok(())
    }

    pub fn print_result(&self, output: &Output) -> Result<()> {
        let mut stdout = io::stdout();
        let mut stderr = io::stderr();

        let title = if output.title.is_empty() {
            String::new()
        } else {
            format!("{}", format!("{}: ", output.title).bold().green())
        };

        self.print_json(&output, stderr.lock())?;

        for data in &output.metadata {
            if let Metadata::Color { color } = data {
                if let Ok(color) = css_color::Srgb::from_str(color) {
                    writeln!(
                        &mut stderr,
                        "{}",
                        "       ".on_truecolor(
                            (color.red * 255.0) as u8,
                            (color.green * 255.0) as u8,
                            (color.blue * 255.0) as u8,
                        )
                    )?;
                }
            }
        }

        for block in &output.blocks {
            match block {
                Block::Comment(comment) => {
                    if self.args.verbose > 0 {
                        writeln!(&mut stderr, "{}{}", title, comment.text)?;
                    }
                }
                Block::Data(data) => {
                    let mut detector = EncodingDetector::new();
                    detector.feed(&data.data, true);
                    if detector.guess(None, true) == encoding_rs::UTF_8 {
                        stdout.write_all(&data.data)?;
                        writeln!(&mut stdout)?;
                    } else {
                        self.print_binary(&data.data)?;
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                Block::Spoiler(spoiler) => {
                    use crossterm::{
                        cursor::MoveToPreviousLine,
                        execute,
                        terminal::{Clear, ClearType},
                    };
                    use std::io::BufRead;
                    let stdin = io::stdin();
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

        self.print_json(&output, stderr.lock())?;

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
        if yozuk_helper_platform::term::is_stdout_tty() {
            let show_color = true;
            let show_char_panel = true;
            let show_position_panel = true;
            let use_squeezing = false;
            let border_style = BorderStyle::Unicode;
            let mut printer = Printer::new(
                &mut stdout,
                show_color,
                show_char_panel,
                show_position_panel,
                border_style,
                use_squeezing,
            );
            printer.print_all(data).unwrap();
        } else {
            stdout.write_all(data)?;
        }
        Ok(())
    }
}
