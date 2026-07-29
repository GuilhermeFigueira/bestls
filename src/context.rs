mod config;
pub(crate) use config::Config;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::{fs, path::PathBuf};
use supports_hyperlinks::Stream;

pub struct AppContext {
    pub(crate) config: config::Config,
    pub(crate) config_path: PathBuf,
    pub(crate) supports_hyperlinks: bool,
}

impl AppContext {
    pub fn load() -> Result<Self> {
        let proj_dirs = Self::create_and_return_project_dir()
            .context("Failed to set up project directories")?;
        let config = Config::load(proj_dirs.config_dir()).context("Failed to load config file")?;
        Ok(Self {
            config,
            config_path: proj_dirs.config_dir().to_path_buf(),
            supports_hyperlinks: supports_hyperlinks::on(Stream::Stdout),
        })
    }

    fn create_and_return_project_dir() -> Result<ProjectDirs> {
        let proj_dirs = ProjectDirs::from("com", "", "bestls")
            .context("Error formulating the project directory")?;

        fs::create_dir_all(proj_dirs.config_dir()).with_context(|| {
            format!(
                "Error creating the project config directory: {:?}",
                proj_dirs.config_dir()
            )
        })?;

        Ok(proj_dirs)
    }
}
