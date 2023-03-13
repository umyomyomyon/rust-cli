use anyhow::Context;
use clap::Parser;
use grrs::{ validate_pattern, find_matches };

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    validate_pattern(&args.pattern).map_err(|err| format!("{}", err))?;
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file: {}", args.path.display()))?;

    find_matches(&content, &args.pattern, std::io::stdout());

    Ok(())
}
