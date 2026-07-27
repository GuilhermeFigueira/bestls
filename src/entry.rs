use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use strum_macros::Display;
use tabled::Tabled;
use unicode_ellipsis;

use crate::context::AppContext;

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

impl Default for FileEntry {
    fn default() -> Self {
        Self {
            name: String::from("<Unknown File>"),
            e_type: EntryType::Unknown,
            len_bytes: 0,
            modified: String::from("-"),
        }
    }
}

#[derive(Debug, Display, Serialize)]
pub enum EntryType {
    File,
    Dir,
    Shortcut,
    Unknown,
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

pub fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, context: &AppContext) {
    match fs::metadata(file.path()) {
        Ok(metadata) => {
            let file_name = file
                .file_name()
                .into_string()
                .unwrap_or_else(|_| FileEntry::default().name);
            data.push(FileEntry {
                name: unicode_ellipsis::truncate_str(
                    &file_name,
                    context.config.display.file_name_size,
                )
                .to_string(),
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
        Err(e) => {
            eprintln!("Error reading this file {:?} : {:?}", file.path(), e);
            data.push(FileEntry::default());
        }
    }
    // TODO: Retornar tamanho de pastas
}

pub fn get_files(path: &Path, context: &AppContext) -> Result<Vec<FileEntry>> {
    let mut data = Vec::default();

    let dir =
        fs::read_dir(path).with_context(|| format!("Error while reading directory: {:?}", path))?;

    for entry in dir {
        let file_result = entry;
        match file_result {
            Ok(file) => map_data(file, &mut data, context),
            Err(_) => data.push(FileEntry::default()),
        }
    }

    Ok(data)
    // TODO: Flag para arquivos e pastas ocultas
    // TODO: Link para abrir a pasta e arquivos no explorador de arquivos
    // TODO: Comando para abrir arquivo
}
