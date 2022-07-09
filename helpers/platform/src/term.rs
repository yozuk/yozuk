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

    const ATTR_QUERY: &[u8] = b"\x1b[c";
    const APC_QUERY: &[u8] = b"\x1b_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1b\\";
    const OK: &str = ";OK\x1b\\";

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

            stdout.flush()?;
        }

        stdout.write_all(b"\x1b_Gm=0;\x1b\\\n")?;
        stdout.flush()?;
        Ok(())
    }

    fn is_kitty_image_supported_impl() -> Result<bool> {
        let mut stdout = stdout().lock();
        let mut stdin = stdin().lock();

        enable_raw_mode()?;

        stdout.write_all(ATTR_QUERY)?;
        stdout.flush()?;
        let mut attr_buf = vec![0u8; 128];
        let n = stdin.read(&mut attr_buf)?;
        attr_buf.resize(n, 0);

        stdout.write_all(APC_QUERY)?;
        stdout.write_all(ATTR_QUERY)?;
        stdout.flush()?;
        let mut apc_buf = vec![0u8; 128];
        let n = stdin.read(&mut apc_buf)?;
        apc_buf.resize(n, 0);

        disable_raw_mode()?;

        if apc_buf.len() <= attr_buf.len() {
            return Ok(false);
        }

        Ok(String::from_utf8(apc_buf)
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

pub use term_is_tty::*;
pub use term_kitty_image::*;
