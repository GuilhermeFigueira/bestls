mod file_entry;
use file_entry::FileEntry;
use std::{fs::Metadata, os::windows::prelude::*};

use anyhow::{Context, Result};
use std::{
    fs::{self},
    path::Path,
};

use crate::context::AppContext;

pub fn is_hidden(metadata: &Metadata) -> std::io::Result<bool> {
    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

    let attributes = metadata.file_attributes();

    Ok((attributes & FILE_ATTRIBUTE_HIDDEN) > 0)
}

pub fn map_data(
    file: fs::DirEntry,
    data: &mut Vec<FileEntry>,
    context: &AppContext,
    show_all: bool,
) {
    match fs::metadata(file.path()) {
        Ok(metadata) => {
            if show_all || context.config.display.show_hidden {
                data.push(FileEntry::get(context, file, &metadata));
            } else if !is_hidden(&metadata).unwrap_or(false) {
                data.push(FileEntry::get(context, file, &metadata));
            }
        }
        Err(e) => {
            eprintln!("Error reading this file {:?} : {:?}", file.path(), e);
            data.push(FileEntry::default());
        }
    }
}

pub fn get_files(path: &Path, context: &AppContext, show_all: bool) -> Result<Vec<FileEntry>> {
    let mut data = Vec::default();

    let dir =
        fs::read_dir(path).with_context(|| format!("Error while reading directory: {:?}", path))?;

    for entry in dir {
        let file_result = entry;
        match file_result {
            Ok(file) => map_data(file, &mut data, context, show_all),
            Err(_) => data.push(FileEntry::default()),
        }
    }

    Ok(data)
}
