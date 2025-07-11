// src/main.rs
/*
 * Main executable for GPTCryptoSuiteKit
 */

use clap::Parser;
use gptcryptosuitekit::{Result, run};

#[derive(Parser)]
#[command(version, about = "GPTCryptoSuiteKit - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
