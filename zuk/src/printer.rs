use anyhow::Result;
use console::Term;
use content_inspector::ContentType;
use hexyl::{BorderStyle, Printer};
use owo_colors::OwoColorize;
use std::io::Write;
use yozuk_sdk::prelude::*;

pub struct TerminalPrinter;

impl TerminalPrinter {
    pub fn new() -> Self {
        Self
    }

    pub fn print_commands(&self, commands: &[CommandArgs]) -> Result<()> {
        let mut stdout = Term::stdout();
        for cmd in commands {
            writeln!(&mut stdout, "{}", shell_words::join(&cmd.args))?;
        }
        Ok(())
    }

    pub fn print_result(&self, output: &Output) -> Result<()> {
        let mut stdout = Term::stdout();
        let mut stderr = Term::stderr();

        let title = if output.title.is_empty() {
            String::new()
        } else {
            format!("{}", format!("{}: ", output.title).bold().green())
        };

        for block in &output.blocks {
            match block {
                Block::Comment(comment) => {
                    writeln!(&mut stderr, "{}{}", title, comment.text)?;
                }
                Block::Data(data) => {
                    let data = data.data.data().unwrap();
                    let printable = content_inspector::inspect(data) == ContentType::UTF_8;
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
                _ => writeln!(&mut stderr, "{}", "[unimplemented]".dimmed())?,
            }
        }

        Ok(())
    }

    pub fn print_error(&self, outputs: &[Output]) -> Result<()> {
        let mut stderr = Term::stderr();

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
        let mut stderr = Term::stderr();
        writeln!(&mut stderr, "{}", err.red())?;
        Ok(())
    }

    pub fn print_suggest(&self, suggest: &str) -> Result<()> {
        let mut stderr = Term::stderr();
        writeln!(&mut stderr, "Did you mean {} ?", suggest.italic())?;
        Ok(())
    }

    fn print_binary(&self, data: &[u8]) -> Result<()> {
        let mut stdout = Term::stdout();
        if console::user_attended() {
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
