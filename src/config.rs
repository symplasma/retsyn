use confique::Config;
use confique::toml;
use std::fs;
use std::io;
use std::path::PathBuf;

pub(crate) type PathList = Vec<PathBuf>;

#[derive(Config)]
pub struct Conf {
    /// Directories containing loose markdown files to index
    #[config(default = ["~/Markor"])]
    pub(crate) markdown_files: PathList,
}

impl Conf {
    /// Get the default config file path
    pub fn config_path() -> PathBuf {
        use crate::retsyn_app::PROJECT_DIRS;
        PROJECT_DIRS.config_dir().to_path_buf().join("retsyn.toml")
    }

    /// Write the default config template to the config file path if it doesn't exist
    pub fn write_default_config() -> io::Result<PathBuf> {
        let config_path = Self::config_path();

        if config_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Config file already exists at: {}", config_path.display()),
            ));
        }

        // Create parent directories if they don't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Generate the default config template
        let template = toml::template::<Conf>(confique::toml::FormatOptions::default());

        // Write to file
        fs::write(&config_path, template)?;

        Ok(config_path)
    }
}
