use crate::term;
use crate::Args;
use anyhow::Result;
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
                    if yozuk_helper_filetype::is_utf8_text(&data.data) {
                        stdout.write_all(&data.data)?;
                        writeln!(&mut stdout)?;
                    } else {
                        self.print_binary(&data.data, &data.file_name, &data.media_type)?;
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

    fn print_image(&self, data: &[u8], file_name: &str, media_type: &MediaTypeBuf) -> Result<bool> {
        if (media_type == media_type!(IMAGE / PNG)
            || media_type == media_type!(IMAGE / GIF)
            || media_type == media_type!(IMAGE / JPEG))
            && term::is_iterm2_image_supported()
        {
            term::iterm2_image_show(data, Some(file_name))?;
            return Ok(true);
        }
        if media_type == media_type!(IMAGE / PNG) && term::is_kitty_image_supported() {
            term::kitty_image_show_png(data)?;
            return Ok(true);
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let Ok((width, height)) = hanbun::size() {
            if let Ok(image) = image::load_from_memory(data) {
                use hanbun::Color;

                let ratio_x = (image.width() as f64 / width as f64).ceil() as u32;
                let ratio_y = (image.height() as f64 / height as f64).ceil() as u32;
                let ratio_x = if ratio_x % 2 == 1 {
                    ratio_x + 1
                } else {
                    ratio_x
                };
                let ratio_y = if ratio_y % 2 == 1 {
                    ratio_y + 1
                } else {
                    ratio_y
                };
                let ratio = ratio_x.min(ratio_y);
                let resized_image = image.resize_exact(
                    (image.width() as f64 / ratio as f64) as u32,
                    (image.height() as f64 / ratio as f64) as u32,
                    image::imageops::FilterType::Nearest,
                );

                let mut buffer = hanbun::Buffer::new(
                    resized_image.width() as _,
                    (resized_image.height() / 2) as _,
                    ' ',
                );
                for (y, row) in resized_image.to_rgb8().rows().enumerate() {
                    for (x, pixel) in row.enumerate() {
                        hanbun::Buffer::color(
                            &mut buffer,
                            x,
                            y,
                            Color::Rgb {
                                r: pixel[0],
                                g: pixel[1],
                                b: pixel[2],
                            },
                        );
                    }
                }
                buffer.draw();
                buffer.clear(' ');

                return Ok(true);
            }
        }

        Ok(false)
    }

    fn print_binary(&self, data: &[u8], file_name: &str, media_type: &MediaTypeBuf) -> Result<()> {
        let mut stdout = io::stdout();
        if term::is_stdout_tty() {
            if self.print_image(data, file_name, media_type)? {
                return Ok(());
            }
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
