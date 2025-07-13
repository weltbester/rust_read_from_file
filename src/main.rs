use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
