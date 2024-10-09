use std::io::{self, Write};

use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line)?;
        }
    }

    Ok(())
}
