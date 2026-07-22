
use std::{fs, io, path::PathBuf};

use directories::ProjectDirs;
use owo_colors::OwoColorize;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub(crate) display: Display,
}

impl Config {
    pub fn get() -> Result<Config, Box<dyn std::error::Error>> {
        let proj_dirs = supper::create_and_return_project_dir()?;
        let config_file_path = PathBuf::from(&proj_dirs.config_dir().join("config.toml"));
        let config: Config = match fs::exists(config_file_path) {
            Ok(true) => ,
            Ok(false) => Config {
                        display: Display {
                            show_hidden: true,
                            show_folder_size: true,
                        },
                    },
            Err(e) => println!("erro "),
        };
        config
    }
    
}
#[derive(Deserialize, Debug)]
pub struct Display {
    pub(crate) show_hidden: bool,
    pub(crate) show_folder_size: bool,
}




// match fs::write(config_file_path) {
//             Ok(()) => {
//                 let config = Config {
//                     display: Display {
//                         show_hidden: true,
//                         show_folder_size: true,
//                     },
//                 };
//             }
//             Err(e) => println!("erro"),
//         },