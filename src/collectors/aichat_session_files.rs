use ignore::WalkBuilder;
use serde::Deserialize;
use tracing::{debug, warn};

use crate::{
    config::PathList,
    model::index_entry::{IndexEntry, IndexPath, IndexPathSender},
};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
struct Session {
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct Message {
    role: String,
    content: String,
}

pub(crate) const AICHAT_SESSION_FILES_SOURCE: &str = "aichat_session_files";

pub(crate) struct AichatSessionFiles {
    paths: PathList,
}

impl AichatSessionFiles {
    /// Creates a new AichatSessionFiles object that holds the paths for the WebScrapbook archive files.
    ///
    /// Tildes in the config are expanded on construction.
    pub(crate) fn new(path_list: &PathList) -> Self {
        let paths = path_list
            .iter()
            .map(|p| {
                // expanding tildes into absolute paths and adding the data subdir to each path here
                let path = PathBuf::from(shellexpand::tilde(&p.to_string_lossy()).into_owned());
                let unnamed_path = path.join("data");
                [path, unnamed_path]
            })
            .flatten()
            .collect();

        AichatSessionFiles { paths }
    }

    /// Collects all of the entries and sends them to the indexer.
    ///
    /// This method does this in a separate thread.
    pub(crate) fn collect_entries(&self, sender: IndexPathSender) {
        for dir in &self.paths {
            // we're limiting the depth since these entries are all stored in the same dir
            for result in WalkBuilder::new(dir)
                .max_depth(Some(1))
                .filter_entry(|d| d.path().extension().map(|e| e == "yaml").unwrap_or(false))
                .build()
            {
                match result {
                    // TODO switch to the tracing crate
                    Err(e) => {
                        // TODO collect these errors so the user can see what is not being indexed properly
                        warn!("could not open path: {}", e)
                    }

                    Ok(entry) => {
                        let aichat_session_file = entry.path();
                        if aichat_session_file.exists() && aichat_session_file.is_file() {
                            // TODO check the file type here
                            // if this is a file, send it to the fulltext index to check if it is already indexed and up to date
                            debug!("sending path {}...", aichat_session_file.to_string_lossy());
                            sender
                                .send(IndexPath::AichatSessionFile(
                                    aichat_session_file.to_path_buf(),
                                ))
                                .expect("should be able to send new entries to index");
                        }
                    }
                }
            }
        }

        // once we are done, close the channel
        drop(sender);
    }

    pub(crate) fn convert_path_to_entry(path: &Path) -> IndexEntry {
        debug!(
            "attempting to convert {} to entry...",
            path.to_string_lossy()
        );

        // TODO properly handle non UTF-8 file contents
        // TODO handle very large files efficiently, maybe switch to a streaming library
        let body = fs::read_to_string(&path).unwrap_or_default();

        // Deserialize the YAML into our Session structure.
        // TODO replace `from_str` with `from_reader`
        let session: Session =
            serde_yaml::from_str(&body).expect("should be able to deserialize yaml");

        // Extract title according to priority rules
        let title = Self::extract_title(&path);

        // Iterate over the messages and print them in Markdown format.
        let message_vec: Vec<String> = session
            .messages
            .iter()
            .map(|message| {
                vec![
                    format!("# ---{}--- #\n", message.role),
                    message.content.clone(),
                ]
            })
            .flatten()
            .collect();

        IndexEntry::new(
            AICHAT_SESSION_FILES_SOURCE.to_owned(),
            path.to_string_lossy().to_string(),
            title,
            message_vec.join("\n"),
        )
    }

    fn extract_title(path: &&Path) -> String {
        path.file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or("UNKNOWN_TITLE".to_string())
    }
}
