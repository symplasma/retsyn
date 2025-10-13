use std::path::PathBuf;

use confique::Config;

pub(crate) type PathList = Vec<PathBuf>;

#[derive(Config)]
pub(crate) struct Conf {
    /// Directories containing loose markdown files to index
    #[config(default = ["~/Markor"])]
    pub(crate) markdown_files: PathList,
}
