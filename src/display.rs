use crate::entry::get_files;
use osc8::Hyperlink;
use std::path::PathBuf;
use tabled::{
    Table,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

pub fn print_table(path: PathBuf) {
    let files = get_files(&path);
    let mut table = Table::new(files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", table)
}

pub fn print_title(path: &PathBuf, supports_hyperlinks: bool) {
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
    // TODO: Seta para voltar para a pasta pai (caso exista)
}
