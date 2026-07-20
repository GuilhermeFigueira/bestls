use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Best Ls command ever")]
pub struct Cli {
    /// Path to the folder to list (defaults to the current directory)
    pub(crate) path: Option<PathBuf>,

    /// Parse the output to json
    #[arg(short, long)]
    pub(crate) json: bool,

    /// Toggle default hidden-file visibility (does not list files)
    #[arg(short('A'), long)]
    pub(crate) toggle_all: bool,

    /// Toggle whether folder sizes are calculated and displayed
    #[arg(short('F'), long)]
    pub(crate) toggle_show_folder_size: bool,

    /// Show all files
    #[arg(short, long)]
    pub(crate) all: bool,
}
