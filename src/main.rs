use std::fs::{self, File};
use std::io::{BufReader, BufRead};

use anyhow::{Result, Context};
use bstr::io::BufReadExt;

fn main() -> Result<()> {
    let path = std::env::args().nth(1).context("first arg must be file")?;
    let file = File::open(path).context("failed to open file")?;
    let file = BufReader::new(file);

    for (line_number, line) in file.byte_lines().enumerate() {
        let line_number = line_number + 1;
        let line = line.context("failed to read line")?;
        let line = String::from_utf8(line);

        if let Err(err) = line {
            let error_at = err.utf8_error().valid_up_to() + 1;
            eprintln!("invalid utf-8 at line {line_number}, column {error_at}");
        }
    }

    Ok(())
}
