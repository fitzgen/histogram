//! Reads samples from stdin, one per line, and then prints the resulting
//! histogram.

extern crate histogram;

use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    if let Err(e) = try_main() {
        let mut stderr = io::stderr();
        let _ = write!(&mut stderr, "error: {}", e);
        process::exit(1);
    }
}

fn try_main() -> io::Result<()> {
    let mut hist = histogram::Histogram::default();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    while stdin.read_line(&mut line)? > 0 {
        let sample = line.trim()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        hist.increment(sample)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        line.clear();
    }

    println!("{}", hist);
    Ok(())
}
