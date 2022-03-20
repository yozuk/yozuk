use clap::{ArgEnum, Parser};
use std::path::PathBuf;

/// Assistant Bot for Programmers
#[derive(Parser)]
#[clap(name = "zuk", trailing_var_arg = true)]
pub struct Args {
    /// Request query
    #[clap(multiple_occurrences(true))]
    pub query: Vec<String>,

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
    #[clap(long)]
    pub config: Option<PathBuf>,

    /// Start as a REST server
    #[cfg(feature = "server")]
    #[clap(long)]
    pub server: Option<std::net::SocketAddr>,
}

#[derive(ArgEnum, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Term,
    Json,
}
