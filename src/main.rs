mod cli;
mod display;
mod entry;
mod project;

use clap::Parser;
use cli::Cli;
use config::Config;
use display::{print_table, print_title};
use entry::get_files;
use owo_colors::OwoColorize;
use std::{fs, path::PathBuf};
use supports_hyperlinks::Stream;

fn main() {
    let config = Config::get();
    let cli = Cli::parse();
    // // NOTE: Caso programa escalar, transformar em varíavel global com lazylock, ou um "config"
    // let supports_hyperlinks: bool = supports_hyperlinks::on(Stream::Stdout);

    // let path = cli.path.unwrap_or(PathBuf::from("."));

    // if let Ok(does_exist) = fs::exists(&path) {
    //     if does_exist {
    //         if cli.json {
    //             let files = get_files(&path);
    //             println!(
    //                 "{}",
    //                 serde_json::to_string(&files).unwrap_or("cannot parse json".red().to_string())
    //             );
    //         } else {
    //             print_title(&path, supports_hyperlinks);
    //             print_table(path);
    //         }
    //     } else {
    //         println!("{}", "Path does not exist".red())
    //     }
    // } else {
    //     println!("{}", "Error reading directory".red())
    // }
}
