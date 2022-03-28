use anyhow::Result;
use console::Style;
use console::Term;
use content_inspector::ContentType;
use hexyl::{BorderStyle, Printer};
use mediatype::names::JSON;
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

    pub fn print_error(&self, outputs: &[Output]) -> Result<()> {
        let mut stderr = Term::stderr();

        let output = &outputs[0];
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
        let mut stderr = Term::stderr();

        let style = if console::colors_enabled() {
            Style::new().red()
        } else {
            Style::new()
        };
        writeln!(&mut stderr, "{}", style.apply_to(err))?;
        Ok(())
    }

    pub fn print_suggest(&self, suggest: &str) -> Result<()> {
        let mut stderr = Term::stderr();

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
