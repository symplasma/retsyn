use std::path::PathBuf;

use ignore::Walk;

use crate::config::{Conf, PathList};

pub(crate) struct FulltextIndex {
    markdown_files: PathList,
}

impl FulltextIndex {
    pub(crate) fn new(config: &Conf) -> Self {
        Self {
            markdown_files: config
                .markdown_files
                .iter()
                // expand tildes into absolute paths
                .map(|p| PathBuf::from(shellexpand::tilde(&p.to_string_lossy()).into_owned()))
                .collect(),
        }
    }

    pub(crate) fn update(&self) {
        for dir in &self.markdown_files {
            for result in Walk::new(dir) {
                match result {
                    // TODO replace the
                    Ok(entry) => println!("{}", entry.path().to_string_lossy()),
                    Err(e) => println!("could not open path: {}", e),
                }
            }
        }
    }
}
