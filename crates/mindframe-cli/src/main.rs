use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "mindframe", version, about = "Turn knowledge into visual stories")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Build a MindFrame project from a Markdown knowledge source.
    Build {
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Build { input } => build(input),
    }
}

fn build(input: PathBuf) -> Result<()> {
    // 1. Validate the external input before any pipeline work.
    if input.extension().and_then(|ext| ext.to_str()) != Some("md") {
        bail!("input must be a Markdown (.md) file");
    }

    let source = std::fs::read_to_string(&input)
        .with_context(|| format!("failed to read '{}'", input.display()))?;

    if source.trim().is_empty() {
        bail!("input Markdown must not be empty");
    }

    // 2. Stop at the current V0.1 boundary until extraction is implemented.
    println!(
        "loaded '{}' ({} bytes); knowledge extraction is the next pipeline stage",
        input.display(),
        source.len()
    );

    Ok(())
}
