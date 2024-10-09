use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let file = File::open(args.path).expect("could not open file");
    let mut reader = BufReader::new(file);
    let mut string = String::new();

    while reader.read_line(&mut string).unwrap() > 0 {
        if string.contains(&args.pattern) {
            println!("{}", string);
        }

        string.clear();
    }
}
