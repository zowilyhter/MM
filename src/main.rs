// src/main.rs
/*
 * Main executable for M2M100
 */

use clap::Parser;
use m2m100::{Result, run};

#[derive(Parser)]
#[command(version, about = "M2M100 - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
