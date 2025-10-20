use ignore::Walk;

use crate::{
    config::PathList,
    index_entry::index_entry::{IndexPath, IndexPathSender},
};
use std::path::PathBuf;

pub(crate) const MARKDOWN_FILES_SOURCE: &str = "markdown_files";

pub(crate) struct MarkdownFiles {
    sender: IndexPathSender,
    paths: PathList,
}

impl MarkdownFiles {
    /// Creates a new MarkdownFiles object that holds the paths for raw markdown files.
    ///
    /// Tildes in the config are expanded on construction.
    pub(crate) fn new(sender: IndexPathSender, path_list: &PathList) -> Self {
        let paths = path_list
            .iter()
            // expand tildes into absolute paths
            .map(|p| PathBuf::from(shellexpand::tilde(&p.to_string_lossy()).into_owned()))
            .collect();

        MarkdownFiles { sender, paths }
    }

    /// Collects all of the entries and sends them to the indexer.
    ///
    /// This method does this in a separate thread.
    pub(crate) fn collect_entries(&self) {
        for dir in &self.paths {
            for result in Walk::new(dir) {
                match result {
                    // TODO switch to the tracing crate
                    Err(e) => {
                        // TODO collect these errors so the user can see what is not being indexed properly
                        println!("could not open path: {}", e)
                    }

                    Ok(entry) => {
                        if entry.file_type().map(|e| e.is_file()).unwrap_or(false) {
                            // TODO check the file type here
                            // if this is a file, send it to the fulltext index to check if it is already indexed and up to date
                            self.sender
                                .send(IndexPath::MarkdownFile(entry.path().to_path_buf()))
                                .expect("should be able to send new entries to index");
                        }
                    }
                }
            }
        }
    }
}
