use super::{json::*, Args, OutputFormat};
use anyhow::Result;
use console::Style;
use console::Term;
use content_inspector::ContentType;
use hexyl::{BorderStyle, Printer};
use mediatype::names::JSON;
use std::io::Write;
use yozuk_sdk::prelude::*;

pub struct TerminalPrinter<'a> {
    args: &'a Args,
}

impl<'a> TerminalPrinter<'a> {
    pub fn new(args: &'a Args) -> Self {
        Self { args }
    }

    pub fn print_commands(&self, commands: &[CommandArgs]) -> Result<()> {
        let mut stdout = Term::stdout();
        match self.args.output {
            OutputFormat::Term => {
                for cmd in commands {
                    writeln!(&mut stdout, "{}", shell_words::join(&cmd.args))?;
                }
            }
            OutputFormat::Json => {
                serde_json::to_writer(&mut stdout, &JsonResult::Commands { commands })?;
                writeln!(&mut stdout)?;
            }
        }
        Ok(())
    }

    pub fn print_result(&self, output: &Output) -> Result<()> {
        let mut stdout = Term::stdout();
        let mut stderr = Term::stderr();

        if self.args.output == OutputFormat::Json {
            serde_json::to_writer(&mut stdout, &JsonResult::Ok { output })?;
            writeln!(&mut stdout)?;
            return Ok(());
        }

        for section in &output.sections {
            match section.kind {
                SectionKind::Comment => {
                    if !output.module.is_empty() {
                        let style = if console::colors_enabled() {
                            Style::new().bold().green()
                        } else {
                            Style::new()
                        };
                        write!(
                            &mut stderr,
                            "{}",
                            style.apply_to(&format!("{}: ", output.module))
                        )?;
                    }
                    let style = if console::colors_enabled() {
                        Style::new().dim().white()
                    } else {
                        Style::new()
                    };
                    writeln!(&mut stderr, "{}", style.apply_to(section.as_utf8()))?;
                }
                SectionKind::Value => {
                    if section.media_type.suffix() == Some(JSON) {
                        if let Ok(value) =
                            serde_json::from_slice::<serde_json::Value>(&section.data)
                        {
                            if let Ok(yaml) = serde_yaml::to_string(&value) {
                                stdout.write_str(yaml.trim_start_matches("---\n"))?;
                                return Ok(());
                            }
                        }
                    }
                    let printable = content_inspector::inspect(&section.data) == ContentType::UTF_8;
                    if printable {
                        stdout.write_all(&section.data)?;
                        writeln!(&mut stdout)?;
                    } else {
                        self.print_binary(&section.data)?;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn print_error(&self, output: &Output) -> Result<()> {
        let mut stdout = Term::stdout();
        let mut stderr = Term::stderr();

        if self.args.output == OutputFormat::Json {
            serde_json::to_writer(&mut stdout, &JsonResult::Fail { output })?;
            writeln!(&mut stdout)?;
            return Ok(());
        }

        for section in &output.sections {
            if !output.module.is_empty() {
                let style = if console::colors_enabled() {
                    Style::new().bold().red()
                } else {
                    Style::new()
                };
                write!(
                    &mut stderr,
                    "{}",
                    style.apply_to(&format!("{}: ", output.module))
                )?;
            }
            writeln!(&mut stderr, "{}", section.as_utf8())?;
        }

        Ok(())
    }

    pub fn print_error_str(&self, err: &str) -> Result<()> {
        let mut stdout = Term::stdout();
        let mut stderr = Term::stderr();

        if self.args.output == OutputFormat::Json {
            serde_json::to_writer(&mut stdout, &JsonResult::Error { message: err })?;
            writeln!(&mut stdout)?;
            return Ok(());
        }

        let style = if console::colors_enabled() {
            Style::new().red()
        } else {
            Style::new()
        };
        writeln!(&mut stderr, "{}", style.apply_to(err))?;
        Ok(())
    }

    pub fn print_suggest(&self, suggest: &str) -> Result<()> {
        let mut stdout = Term::stdout();
        let mut stderr = Term::stderr();

        if self.args.output == OutputFormat::Json {
            serde_json::to_writer(&mut stdout, &JsonResult::Suggest { suggest })?;
            writeln!(&mut stdout)?;
            return Ok(());
        }

        let style = if console::colors_enabled() {
            Style::new().italic()
        } else {
            Style::new()
        };
        writeln!(&mut stderr, "Did you mean {} ?", style.apply_to(suggest))?;
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
