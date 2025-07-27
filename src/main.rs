// src/main.rs
/*
 * Main executable for ModernNano
 */

use clap::Parser;
use modernnano::{Result, run};

#[derive(Parser)]
#[command(version, about = "ModernNano - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
