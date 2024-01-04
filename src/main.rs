use std::fs::File;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Source file to be compiled.
    source: PathBuf,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // Path of the source code to open the file.
    let file = File::open(cli.source)?;

    // Now run it through cradle!

    //testing
    let _ = parser::regexes("Hi there!");

    Ok(()) // Happy path!
}

pub mod ops;
pub mod parser;
