use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Source file to be compiled.
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // Path of the source code.
    let source_path = cli.source;

    let (token, advance) = crate::parser::char_parse_operator('/', '=');
    dbg!(token, advance);
}

pub mod ops;
pub mod parser;
