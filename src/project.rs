mod config;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;

pub struct Project {
    pub(crate) config: Config,
}

impl Project {
    pub fn create_and_return_project_dir() -> Result<ProjectDirs> {
        let proj_dirs = ProjectDirs::from("com", "", "bestls")
            .context("Error formulating the project directory")?;
        fs::create_dir_all(proj_dirs.config_dir()).with_context(|| {
            format!(
                "Error creating the project config directory: {:?}",
                proj_dirs.config_dir()
            )
        });
        Ok(proj_dirs)
    }
}
