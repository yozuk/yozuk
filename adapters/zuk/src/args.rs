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

    /// Copy output to the clipboard
    #[clap(short, long)]
    pub clipboard: bool,

    /// Load config from a TOML file
    #[clap(long)]
    pub config: Option<PathBuf>,
}

#[derive(ArgEnum, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Term,
    Json,
}
