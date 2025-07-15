use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf, // PathBuf works cross-platform
}

// This program reads a file line by line and prints each character
fn main() -> io::Result<()> {
    let file = File::open("src/main.rs")?;
    let reader = BufReader::new(file);

    // reader.lines() is an iterator over the lines of the file
    // and returns Result<String, io::Error>
    for line_result in reader.lines() {
	// Unwrap the Result to get the line
        let line = line_result?;
	// println!("{}", line);
	for char in line.chars() {
	    print!("{}", char);
	}
	println!(); // Print a newline after each line
    }
    Ok(())
}

// This program reads the whole file into a string
fn main() -> Result<()> {
    let args = Cli::parse();

    // let content = std::fs::read_to_string(&args.path).expect("Failed to read file"); // Readsfile into a string
    let content = std::fs::read_to_string(&args.path)
	.with_context(|| format!("could not read file: '{}'", args.path.display()))?;
    
    for line in content.lines() {
	if line.contains(&args.pattern) {
	    println!("{}", line);
	}
    }
    Ok(())
}
