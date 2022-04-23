use anyhow::Result;
use console::Term;
use content_inspector::ContentType;
use hexyl::{BorderStyle, Printer};
use mediatype::names::JSON;
use owo_colors::OwoColorize;
use std::io::Write;
use std::str::FromStr;
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
                    if !output.title.is_empty() {
                        write!(
                            &mut stderr,
                            "{}",
                            format!("{}: ", output.title).bold().green()
                        )?;
                    }
                    writeln!(&mut stderr, "{}", section.as_utf8().dimmed().white())?;
                }
                SectionKind::Value => {
                    let color = section
                        .attrs
                        .get("com.yozuk.preview.color")
                        .and_then(|color| color.as_str())
                        .and_then(|color| css_color::Srgb::from_str(color).ok());
                    if let Some(color) = color {
                        writeln!(
                            &mut stderr,
                            "{}",
                            "       ".on_truecolor(
                                (color.red * 255.0) as u8,
                                (color.green * 255.0) as u8,
                                (color.blue * 255.0) as u8
                            )
                        )?;
                    }
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
            if !output.title.is_empty() {
                write!(
                    &mut stderr,
                    "{}",
                    format!("{}: ", output.title).bold().red()
                )?;
            }
            writeln!(&mut stderr, "{}", section.as_utf8())?;
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
