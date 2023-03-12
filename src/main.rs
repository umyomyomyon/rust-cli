use std::fs::File;
use std::io::{BufReader, BufRead};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()>{
    let args = Cli::parse();
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}


