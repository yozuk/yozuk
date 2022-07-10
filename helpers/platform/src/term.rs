#[cfg(not(target_os = "wasi"))]
mod term_is_tty {
    use atty::Stream;

    pub fn is_stdin_tty() -> bool {
        atty::is(Stream::Stdin)
    }

    pub fn is_stdout_tty() -> bool {
        atty::is(Stream::Stdout)
    }

    pub fn is_stderr_tty() -> bool {
        atty::is(Stream::Stderr)
    }
}

#[cfg(target_os = "wasi")]
mod term_is_tty {
    fn is_tty(fd: wasi::Fd) -> bool {
        unsafe {
            wasi::fd_fdstat_get(fd).map(|stat| stat.fs_filetype.name()) == Ok("CHARACTER_DEVICE")
        }
    }

    pub fn is_stdin_tty() -> bool {
        is_tty(wasi::FD_STDIN)
    }

    pub fn is_stdout_tty() -> bool {
        is_tty(wasi::FD_STDOUT)
    }

    pub fn is_stderr_tty() -> bool {
        is_tty(wasi::FD_STDERR)
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod term_kitty_image {
    use anyhow::Result;
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
    use std::io::{stdin, stdout, Read, Write};

    const KITTY_QUERY: &[u8] = b"\x1b_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1b\\\x1b[5n";
    const OK: &str = ";OK\x1b\\";
    const SUFFIX: &[u8] = b"\x1b[0n";

    pub fn is_kitty_image_supported() -> bool {
        is_kitty_image_supported_impl().unwrap_or(false)
    }

    pub fn kitty_image_show_png(data: &[u8]) -> std::io::Result<()> {
        let mut stdout = stdout().lock();
        let data = base64::encode(data);
        let data = data.as_bytes();
        let mut first = true;
        for chunk in data.chunks(4096) {
            if first {
                first = false;
                stdout.write_all(b"\x1b_Gf=100,a=T,m=1;")?;
            } else {
                stdout.write_all(b"\x1b_Gm=1;")?;
            }
            stdout.write_all(chunk)?;
            stdout.write_all(b"\x1b\\")?;
        }

        stdout.write_all(b"\x1b_Gm=0;\x1b\\\n")?;
        stdout.flush()?;
        Ok(())
    }

    fn is_kitty_image_supported_impl() -> Result<bool> {
        let mut stdout = stdout().lock();
        let mut stdin = stdin().lock();

        enable_raw_mode()?;

        stdout.write_all(KITTY_QUERY)?;
        stdout.flush()?;

        let mut buf = vec![0u8; 0];
        loop {
            let len = buf.len();
            buf.resize(len + 128, 0);

            let n = stdin.read(&mut buf[len..])?;
            buf.resize(len + n, 0);

            if buf.iter().rev().take(SUFFIX.len()).eq(SUFFIX.iter().rev()) {
                break;
            }
        }

        disable_raw_mode()?;

        Ok(String::from_utf8(buf)
            .ok()
            .and_then(|s| s.find(OK))
            .is_some())
    }
}

#[cfg(target_arch = "wasm32")]
mod term_kitty_image {
    pub fn is_kitty_image_supported() -> bool {
        false
    }

    pub fn kitty_image_show_png(_data: &[u8]) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod term_iterm2_image {
    use anyhow::Result;
    use base64::write::EncoderWriter;
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
    use semver::{Version, VersionReq};
    use std::io::{stdin, stdout, Read, Write};

    const ITERM2_QUERY: &[u8] = b"\x1b[1337n\x1b[5n";
    const PREFIX: &str = "\x1b[ITERM2 ";
    const SUFFIX: &[u8] = b"\x1b[0n";

    pub fn is_iterm2_image_supported() -> bool {
        let req = VersionReq::parse(">=2.9").unwrap();
        if let Ok(version) = iterm2_version() {
            return req.matches(&version);
        }
        false
    }

    fn iterm2_version() -> Result<Version> {
        let mut stdout = stdout().lock();
        let mut stdin = stdin().lock();

        enable_raw_mode()?;

        stdout.write_all(ITERM2_QUERY)?;
        stdout.flush()?;

        let mut buf = vec![0u8; 0];
        loop {
            let len = buf.len();
            buf.resize(len + 128, 0);

            let n = stdin.read(&mut buf[len..])?;
            buf.resize(len + n, 0);

            if buf.iter().rev().take(SUFFIX.len()).eq(SUFFIX.iter().rev()) {
                break;
            }
        }

        disable_raw_mode()?;

        let version = String::from_utf8(buf)?;
        let version = version
            .trim_start_matches(PREFIX)
            .trim_end_matches("n\x1b[0n");

        Ok(version.parse()?)
    }

    pub fn iterm2_image_show(data: &[u8], name: Option<&str>) -> std::io::Result<()> {
        let mut stdout = stdout().lock();
        stdout.write_all(b"\x1b]1337;File=inline=1")?;
        stdout.write_all(format!(";size={}", data.len()).as_bytes())?;
        if let Some(name) = name {
            stdout.write_all(format!(";name={}", base64::encode(name)).as_bytes())?;
        }
        stdout.write_all(b":")?;
        {
            let mut writer = EncoderWriter::new(&mut stdout, base64::STANDARD);
            writer.write_all(data)?;
            writer.finish()?;
        }
        stdout.write_all(b"\x07\n")?;
        stdout.flush()?;
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
mod term_iterm2_image {
    pub fn is_iterm2_image_supported() -> bool {
        false
    }

    pub fn iterm2_image_show(_data: &[u8]) -> std::io::Result<()> {
        Ok(())
    }
}

pub use term_is_tty::*;
pub use term_iterm2_image::*;
pub use term_kitty_image::*;
