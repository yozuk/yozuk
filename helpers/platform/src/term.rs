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

pub use term_is_tty::*;
