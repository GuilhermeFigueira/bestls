use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug, Serialize)]
#[serde(default)]
pub(crate) struct Config {
    pub(crate) display: Display,
}

impl Config {
    pub(super) fn load(config_path: &Path) -> Result<Self> {
        let config_file_path = PathBuf::from(&config_path.join("config.toml"));

        if fs::exists(&config_file_path).with_context(|| {
            format!(
                "Error checking if config file exists: {:?}",
                &config_file_path
            )
        })? {
            Self::read_existing_config(&config_file_path)
        } else {
            Self::write_default_config(&config_file_path)
        }
    }

    fn read_existing_config(path: &Path) -> Result<Config> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Error reading config file: {:?}", path))?;

        match toml::from_str(&content) {
            Ok(config) => Ok(config),
            Err(e) => {
                eprintln!("Invalid config file, using default options: {}", e);
                Ok(Self::default())
            }
        }
    }

    fn write_default_config(path: &Path) -> Result<Config> {
        Self::default().save(path)?;
        Ok(Self::default())
    }

    pub(crate) fn save(&self, config_path: &Path) -> Result<()> {
        let config_file_path = config_path.join("config.toml");

        let toml_string = toml::to_string(self)
            .with_context(|| format!("Error converting default config to TOML: {:?}", self))?;

        fs::write(&config_file_path, toml_string)
            .with_context(|| format!("Error writing config file: {:?}", &config_file_path))?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: Display::default(),
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(default)]
pub(crate) struct Display {
    pub(crate) show_hidden: bool,
    pub(crate) show_folder_size: bool,
    pub(crate) file_name_length: usize,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            show_hidden: true,
            show_folder_size: true,
            file_name_length: 25,
        }
    }
}
