use clap::Parser;
use std::path::PathBuf;

/// Chatbot for Programmers
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

    /// Increase the verbosity
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,

    /// Start RPC server
    #[cfg(all(feature = "rpc", not(target_arch = "wasm32")))]
    #[clap(long)]
    pub rpc: bool,

    /// Dump embedded model data.
    #[cfg(debug_assertions)]
    #[clap(long, display_order(1000))]
    pub dump_model: Option<PathBuf>,
}
