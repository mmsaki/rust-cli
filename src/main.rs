#![allow(unused)]
use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use std::io::{self, Write};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up");
    warn!("oops, nothing implemented!");
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    grrs::find_matches(&args.pattern, &args.pattern, &mut std::io::stdout());
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

fn answer() -> i32 {
    return (42);
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}
