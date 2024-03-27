use std::fs::File;
use std::io::{BufReader, Read};

use lox_rust::lox;

fn main() {
    // read a file and create a scanner

    let f = File::open("test-data/code_0.txt").expect("Failed to open file");

    let mut reader = BufReader::new(f);

    // for char_result in reader.chars() {
    //     let c = char_result.unwrap();
    //     let i = c as u32;
    //     println!("{} {}", c, i);
    // }

    let mut source = String::new();
    reader
        .read_to_string(&mut source)
        .expect("Failed to read file as String");

    let mut scanner = lox::Scanner::new(source);

    match scanner.scan_tokens() {
        Ok(tokens) => {
            for token in tokens {
                println!("{}", token);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // reader.read_char();

    // let mut line = String::new();
    // let len = reader.read_line(&mut line)?;
    // println!("First line is {len} bytes long");
}
