use std::fs::File;
use std::io::{BufReader, Read};

use lox_rust::lox;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to run
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<(), String> {
    // read a file and create a scanner

    let args = Args::parse();

    let f = File::open(args.file).map_err(|e| e.to_string())?;

    let mut reader = BufReader::new(f);

    let mut source = String::new();
    reader
        .read_to_string(&mut source)
        .map_err(|e| format!("Failed to read file as String: {}", e))?;

    let mut interepreter = lox::Interpreter::new();
    interepreter.execute(source)?;

    Ok(())
}
