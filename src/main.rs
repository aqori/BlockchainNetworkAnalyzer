// src/main.rs
/*
 * Main executable for BlockchainNetworkAnalyzer
 */

use clap::Parser;
use blockchainnetworkanalyzer::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNetworkAnalyzer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
