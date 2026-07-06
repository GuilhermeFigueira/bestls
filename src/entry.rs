use chrono::{DateTime, Utc};
use serde::Serialize;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use strum_macros::Display;
use tabled::Tabled;

#[derive(Debug, Display, Serialize)]
pub enum EntryType {
    File,
    Dir,
    Shortcut,
    Unknown,
}

#[derive(Debug, Tabled, Serialize)]
pub struct FileEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    #[tabled(rename = "Size B")]
    len_bytes: u64,
    #[tabled(rename = "Modified")]
    modified: String,
}

pub fn get_entry_type(metadata: &fs::Metadata, path: &PathBuf) -> EntryType {
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

pub fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
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

pub fn get_files(path: &Path) -> Vec<FileEntry> {
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
