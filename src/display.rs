use crate::{context::AppContext, entry::get_files};
use anyhow::{Context, Ok, Result};
use osc8::Hyperlink;
use std::path::{Path, PathBuf};
use tabled::{
    Table,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

pub fn print_json(path: &Path, context: &AppContext, show_all: bool) -> Result<()> {
    let files = get_files(path, context, show_all)?;
    let json_files = serde_json::to_string(&files).context("Cannot parse JSON")?;
    println!("{}", json_files);
    Ok(())
}

pub fn print_table(path: &Path, context: &AppContext, show_all: bool) -> Result<()> {
    let files = get_files(path, context, show_all)?;
    let mut table = Table::new(files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", table);
    Ok(())
}

pub fn print_title(path: &PathBuf, context: &AppContext) {
    if context.supports_hyperlinks {
        let formatted_link = format!("file:///{}", path.to_string_lossy().replace("\\", "/"));
        let hyperlink = Hyperlink::new(&formatted_link);
        println!("Current path -> {hyperlink}{}{hyperlink:#}", path.display());
    } else {
        println!("Current path -> {}", path.display())
    }
}
