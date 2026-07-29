use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = "Best Ls command ever",
    author = "GuilhermeFigueira"
)]
pub struct Cli {
    /// Path to the folder to list (defaults to the current directory)
    pub(crate) path: Option<PathBuf>,

    /// Parse the output to json
    #[arg(short, long)]
    pub(crate) json: bool,

    /// Show all files
    #[arg(short, long)]
    pub(crate) all: bool,

    #[command(subcommand)]
    pub(crate) config_actions: Option<ConfigAction>,
}

#[derive(Debug, Subcommand)]
pub enum ConfigAction {
    /// Toggle default display behaviors (hidden files, folder sizes) (does not list files)
    Toggle {
        /// Toggle default hidden-file visibility
        #[arg(short('A'), long)]
        all: bool,

        /// Toggle whether folder sizes are calculated and displayed
        #[arg(short('F'), long)]
        folder_size: bool,
    },

    /// Print all settings (does not list files)
    Settings,

    /// Change the default size of file and directory names (does not list files)
    FileNameLength { size: Option<usize> },
}
