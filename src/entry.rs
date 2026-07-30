mod file_entry;
use file_entry::FileEntry;

use anyhow::{Context, Result};
use std::{
    fs::{self},
    path::Path,
};

use crate::context::AppContext;

pub fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, context: &AppContext) {
    match fs::metadata(file.path()) {
        Ok(metadata) => {
            data.push(FileEntry::get(context, file, &metadata));
        }
        Err(e) => {
            eprintln!("Error reading this file {:?} : {:?}", file.path(), e);
            data.push(FileEntry::default());
        }
    }
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
    // TODO: Flag para arquivos e pastas ocultas, atualmente sempre mostra pastas ocultas
}
