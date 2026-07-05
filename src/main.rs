use chrono::{DateTime, Utc};
use clap::Parser;
use osc8::Hyperlink;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use strum_macros::Display;
use supports_hyperlinks::Stream;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
    Shortcut,
    Unknown,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Type"}]
    e_type: EntryType,
    #[tabled{rename="Size B"}]
    len_bytes: u64,
    #[tabled{rename="Modified"}]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Best Ls command ever")]
struct Cli {
    /// Path to the folder to list (defaults to the current directory)
    path: Option<PathBuf>,

    /// Parse the output to json
    #[arg(short, long)]
    json: bool,

    /// Toggle default hidden-file visibility (does not list files)
    #[arg(short, long)]
    all: bool,
}

fn main() {
    let cli = Cli::parse();

    // NOTE: Caso programa escalar, transformar em varíavel global com lazylock, ou um "config"
    let supports_hyperlinks: bool = if supports_hyperlinks::on(Stream::Stdout) {
        true
    } else {
        false
    };

    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if cli.json {
                let files = get_files(&path);
                println!(
                    "{}",
                    serde_json::to_string(&files).unwrap_or("cannot parse json".to_string())
                );
            } else {
                print_title(&path, supports_hyperlinks);
                // TODO: Link para abrir a pasta no explorador de arquivos
                print_table(path);
            }
        } else {
            println!("{}", "Path does not exist".red())
        }
    } else {
        println!("{}", "Error reading directory".red())
    }
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(dir) = fs::read_dir(path) {
        for entry in dir {
            if let Ok(file) = entry {
                map_data(file, &mut data);
            }
        }
    }
    data
    // TODO: Flag para arquivos e pastas ocultas
    // TODO: Link para abrir a pasta e arquivos no explorador de arquivos
    // TODO: Comando para abrir arquivo
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(metadata) = fs::metadata(file.path()) {
        data.push(FileEntry {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("unknown name".into()),
            // FIXME: limitar tamanho de nome de arquivo
            e_type: get_entry_type(&metadata, &file.path()),
            len_bytes: metadata.len(),
            modified: if let Ok(modi) = metadata.modified() {
                let date: DateTime<Utc> = modi.into();
                format!("{}", date.format("%a %b %e %Y"))
                // TODO: Outros formatos
            } else {
                String::default()
            },
        });
    }
    // TODO: Retornar tamanho de pastas
}

fn print_table(path: PathBuf) {
    let files = get_files(&path);
    let mut table = Table::new(files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", table)
}

fn print_title(path: &PathBuf, supports_hyperlinks: bool) {
    match dunce::canonicalize(path) {
        Ok(canonic_path) => {
            if supports_hyperlinks {
                let formatted_link = format!(
                    "file:///{}",
                    canonic_path.to_string_lossy().replace("\\", "/")
                );
                let hyperlink = Hyperlink::new(&formatted_link);
                println!(
                    "Current path: {hyperlink}{}{hyperlink:#}",
                    canonic_path.display()
                );
            } else {
                println!("Current path: {}", canonic_path.display())
            }
        }
        Err(err) => println!("Error while reading current path title: {}", err),
    }
}

fn get_entry_type(metadata: &fs::Metadata, path: &PathBuf) -> EntryType {
    if metadata.is_dir() {
        EntryType::Dir
    } else if metadata.is_file() {
        if path.extension().unwrap_or_default().to_ascii_lowercase() == "lnk" {
            EntryType::Shortcut
        } else {
            EntryType::File
        }
    } else {
        EntryType::Unknown
    }
}
