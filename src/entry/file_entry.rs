use crate::context::AppContext;

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use strum_macros::Display;
use tabled::Tabled;
use unicode_ellipsis;

#[derive(Debug, Tabled, Serialize)]
pub struct FileEntry {
    #[tabled(rename = "Name")]
    pub(crate) name: String,
    #[tabled(rename = "Type")]
    pub(crate) e_type: EntryType,
    #[tabled(rename = "Size B")]
    pub(crate) size: String,
    #[tabled(rename = "Modified")]
    pub(crate) modified: String,
}

impl FileEntry {
    pub fn get(context: &AppContext, file: fs::DirEntry, metadata: &fs::Metadata) -> Self {
        let file_path = &file.path();
        let name = Self::get_name(context, file);
        let e_type = Self::get_entry_type(metadata, file_path);
        let size = Self::get_file_size(context, file_path, &e_type);
        let modified = Self::get_modified(metadata);

        Self {
            name,
            e_type,
            size,
            modified,
        }
    }
    fn get_name(context: &AppContext, file: fs::DirEntry) -> String {
        let file_name = file
            .file_name()
            .into_string()
            .unwrap_or_else(|_| FileEntry::default().name);

        unicode_ellipsis::truncate_str(&file_name, context.config.display.file_name_length)
            .to_string()
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
    fn get_file_size(context: &AppContext, path: &Path, e_type: &EntryType) -> String {
        if matches!(e_type, EntryType::Dir) && !context.config.display.show_folder_size {
            FileEntry::default().size
        } else {
            dir_size::get_size_in_human_bytes(path).unwrap_or_else(|_| FileEntry::default().size)
        }
    }

    fn get_modified(metadata: &fs::Metadata) -> String {
        if let Ok(modi) = metadata.modified() {
            let date: DateTime<Utc> = modi.into();
            format!("{}", date.format("%a %b %e %Y"))
        } else {
            String::default()
        }
    }
}

impl Default for FileEntry {
    fn default() -> Self {
        Self {
            name: String::from("<Unknown File>"),
            e_type: EntryType::Unknown,
            size: String::from("-"),
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
