mod cli;
mod context;
mod display;
mod entry;

use anyhow::{Context, Result, ensure};
use clap::Parser;
use cli::Cli;
use context::AppContext;
use display::{print_json, print_table, print_title};
use owo_colors::OwoColorize;
use std::{fs, path::PathBuf};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", format!("Error: {:?}", e).red());
        std::process::exit(1)
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let context = AppContext::load().context("Error creating the project context")?;

    let path = cli.path.unwrap_or(PathBuf::from("."));
    let canonic_path = dunce::canonicalize(path).context("Error canonicalizing path")?;

    // Ensuring path exists
    ensure!(
        fs::exists(&canonic_path).with_context(|| {
            format!(
                "Error checking if selected path exists: {:?}",
                &canonic_path
            )
        })?,
        "Path does not exist: {:?}",
        canonic_path
    );

    //Ensuring path is a directory
    ensure!(canonic_path.is_dir(), "Path is not a directory");

    // TODO: Adicionar flags

    if cli.json {
        print_json(&canonic_path, &context)?;
    } else {
        print_title(&canonic_path, &context);
        print_table(&canonic_path, &context)?;
    }
    Ok(())
}
