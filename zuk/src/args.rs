use clap::Parser;
use std::path::PathBuf;

/// Assistant Bot for Programmers
#[derive(Parser)]
#[clap(version, trailing_var_arg = true)]
pub struct Args {
    /// Request query
    #[clap(multiple_occurrences(true))]
    pub query: Vec<String>,

    /// Show the inferred internal command without running it
    #[clap(long)]
    pub dry_run: bool,

    /// Run an internal command directly
    #[clap(short, long)]
    pub run: bool,

    /// Specify input files
    #[clap(short, long, multiple_occurrences(true))]
    pub input: Vec<PathBuf>,

    /// Load config from a TOML file
    #[clap(short, long)]
    pub config: Option<PathBuf>,

    /// Start RPC server
    #[cfg(feature = "rpc")]
    #[clap(long)]
    pub rpc: bool,
}
