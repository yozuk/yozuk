use clap::{ArgEnum, Parser};
use std::path::PathBuf;

/// Assistant Bot for Programmers
#[derive(Parser)]
#[clap(author, version, about, trailing_var_arg = true)]
pub struct Args {
    /// Request query
    #[clap(multiple_occurrences(true))]
    pub query: Vec<String>,

    /// Specify the output format
    #[clap(arg_enum, short, long, default_value_t = Mode::Auto)]
    pub mode: Mode,

    /// Specify the output format
    #[clap(arg_enum, short, long, default_value_t = OutputFormat::Term)]
    pub output: OutputFormat,

    /// Increase the logging verbosity
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,

    /// Show the inferred result instead of running it
    #[clap(long)]
    pub dry_run: bool,

    /// Run an internal command directly
    #[clap(short, long)]
    pub run: bool,

    /// Input files
    #[clap(short, long, multiple_occurrences(true))]
    pub input: Vec<PathBuf>,

    /// Load config from a TOML file
    #[clap(short, long)]
    pub config: Option<PathBuf>,

    /// [server] Start as a REST server
    #[cfg(feature = "server")]
    #[clap(long, required_if_eq("mode", "server"), display_order(1000))]
    pub server_addr: Option<std::net::SocketAddr>,

    /// [server] Add an allowed cors origin
    #[cfg(feature = "server")]
    #[clap(
        long,
        display_order(1001),
        requires("server-addr"),
        multiple_occurrences(true)
    )]
    pub cors_origin: Vec<String>,
}

#[derive(ArgEnum, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Term,
    Json,
}

#[derive(ArgEnum, Clone, PartialEq, Eq)]
pub enum Mode {
    Auto,
    Direct,
    Repl,
    #[cfg(feature = "server")]
    HttpServer,
}
