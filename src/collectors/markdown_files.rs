use ignore::Walk;

use crate::{
    config::PathList,
    index_entry::index_entry::{
        IndexEntry, IndexEntrySender, IndexPath, IndexPathReceiver, IndexPathSender,
    },
};
use std::{fs, path::PathBuf};

pub(crate) const MARKDOWN_FILES_SOURCE: &str = "markdown_files";

pub(crate) struct MarkdownFiles {
    paths: PathList,
}

impl MarkdownFiles {
    /// Creates a new MarkdownFiles object that holds the paths for raw markdown files.
    ///
    /// Tildes in the config are expanded on construction.
    pub(crate) fn new(path_list: &PathList) -> Self {
        let paths = path_list
            .iter()
            // expand tildes into absolute paths
            .map(|p| PathBuf::from(shellexpand::tilde(&p.to_string_lossy()).into_owned()))
            .collect();

        MarkdownFiles { paths }
    }

    /// Collects all of the entries and sends them to the indexer.
    ///
    /// This method does this in a separate thread.
    pub(crate) fn collect_entries(&self, sender: IndexPathSender) {
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
                            println!("sending path {}...", entry.path().to_string_lossy());
                            sender
                                .send(IndexPath::MarkdownFile(entry.path().to_path_buf()))
                                .expect("should be able to send new entries to index");
                        }
                    }
                }
            }
        }
        // once we are done, close the channel
        drop(sender);
    }

    /// Extracts the title from markdown content according to priority rules:
    /// 1. Title from frontmatter (if present)
    /// 2. First level 1 heading (# Heading)
    /// 3. Filename without extension
    fn extract_title(content: &str, path: &PathBuf) -> String {
        // Try to extract title from frontmatter
        if let Some(frontmatter_title) = Self::extract_frontmatter_title(content) {
            return frontmatter_title;
        }

        // Try to extract first level 1 heading
        if let Some(heading_title) = Self::extract_first_h1(content) {
            return heading_title;
        }

        // Fall back to filename without extension
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string()
    }

    /// Extracts the title attribute from YAML frontmatter
    fn extract_frontmatter_title(content: &str) -> Option<String> {
        // Check if content starts with frontmatter delimiter
        if !content.starts_with("---") {
            return None;
        }

        // Find the closing delimiter
        let rest = &content[3..];
        let end_pos = rest.find("\n---")?;
        let frontmatter = &rest[..end_pos];

        // Parse frontmatter for title attribute
        for line in frontmatter.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("title:") {
                let title = trimmed[6..].trim();
                // Remove quotes if present
                let title = title.trim_matches(|c| c == '"' || c == '\'');
                if !title.is_empty() {
                    return Some(title.to_string());
                }
            }
        }

        None
    }

    /// Extracts the first level 1 heading from markdown content
    fn extract_first_h1(content: &str) -> Option<String> {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                let heading = trimmed[2..].trim();
                if !heading.is_empty() {
                    return Some(heading.to_string());
                }
            }
        }

        None
    }

    pub(crate) fn convert_paths_to_entries(
        path_receiver: IndexPathReceiver,
        entry_sender: IndexEntrySender,
    ) {
        println!("attempting to convert path to entry...");
        for index_path in path_receiver {
            let new_entry = match index_path {
                IndexPath::MarkdownFile(path_buf) => {
                    // TODO properly handle non UTF-8 file contents
                    // TODO handle very large files efficiently
                    let body = fs::read_to_string(&path_buf).unwrap_or_default();

                    // Extract title according to priority rules
                    let title = Self::extract_title(&body, &path_buf);

                    IndexEntry::new(
                        MARKDOWN_FILES_SOURCE.to_owned(),
                        path_buf.to_string_lossy().to_string(),
                        title,
                        body,
                    )
                }
            };

            // TODO maybe send these in a Box or Arc to reduce memory allocations
            entry_sender
                .send(new_entry)
                .expect("should be able to send new entry to indexer");
        }

        drop(entry_sender);
    }
}
