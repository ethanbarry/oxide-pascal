use std::fs::{self, File};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod ops;
pub mod parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Source file to be compiled.
    source: PathBuf,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // Path of the source code to open the file.
    let file_string = fs::read_to_string(cli.source).unwrap();

    let parsed_file = parser::regexes(&file_string);

    dbg!(parsed_file);

    Ok(()) // Happy path!
}
