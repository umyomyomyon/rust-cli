use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::{Context, Result};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.to_str().unwrap()))?;
    println!("file conetnt: {}", content);
    Ok(())
}


