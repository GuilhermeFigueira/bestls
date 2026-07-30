mod cli;
mod context;
mod display;
mod entry;

use crate::cli::{Cli, ConfigAction};
use crate::context::{AppContext, Config};
use crate::display::{print_json, print_table, print_title};

use anyhow::{Context, Ok, Result, ensure};
use clap::Parser;
use owo_colors::OwoColorize;
use std::path::Path;
use std::{fs, path::PathBuf};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", format!("Error: {:?}", e).red());
        std::process::exit(1)
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let mut context = AppContext::load().context("Error creating the project context")?;

    if let Some(config_actions) = cli.config_actions {
        resolve_config_actions(&config_actions, &mut context.config, &context.config_path)?;
        return Ok(());
    };

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

    // TODO: Adicionar flags: --all

    if cli.json {
        print_json(&canonic_path, &context, cli.all)?;
    } else {
        print_title(&canonic_path, &context);
        print_table(&canonic_path, &context, cli.all)?;
    }
    Ok(())
}

fn resolve_config_actions(
    config_actions: &ConfigAction,
    config: &mut Config,
    config_path: &Path,
) -> Result<()> {
    let needs_saving = apply_config_action(config_actions, config);

    if needs_saving {
        config.save(&config_path)?;
    }
    Ok(())
}

fn apply_config_action(config_actions: &ConfigAction, config: &mut Config) -> bool {
    match config_actions {
        ConfigAction::FileNameLength { size } => {
            if let Some(size) = size {
                config.display.file_name_length = *size;
                println!(
                    "The file name length is now: {}",
                    config.display.file_name_length
                );
                true
            } else {
                println!(
                    "The file name length is: {}",
                    config.display.file_name_length
                );
                false
            }
        }
        ConfigAction::Settings => {
            println!("{}", config);
            false
        }
        ConfigAction::Toggle { all, folder_size } => {
            if *all {
                config.display.show_hidden = !config.display.show_hidden;
                println!(
                    "Showing hidden files is now: {}",
                    config.display.show_hidden
                );
            }
            if *folder_size {
                config.display.show_folder_size = !config.display.show_folder_size;
                println!(
                    "Showing folder size is now: {}",
                    config.display.show_folder_size
                );
            }
            if *all || *folder_size {
                true
            } else {
                println!(
                    "Showing folder size is: {}",
                    config.display.show_folder_size
                );
                println!("Showing hidden files is: {}", config.display.show_hidden);
                false
            }
        }
    }
}
