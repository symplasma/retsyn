use dom_query::Document;
use ignore::WalkBuilder;

use crate::{
    config::PathList,
    index_entry::index_entry::{IndexEntry, IndexPath, IndexPathSender},
};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) const WEB_SCRAPBOOK_FILES_SOURCE: &str = "web_scrapbook_files";

pub(crate) struct WebScrapbookFiles {
    paths: PathList,
}

impl WebScrapbookFiles {
    /// Creates a new WebScrapbookFiles object that holds the paths for the WebScrapbook archive files.
    ///
    /// Tildes in the config are expanded on construction.
    pub(crate) fn new(path_list: &PathList) -> Self {
        let paths = path_list
            .iter()
            .map(|p| {
                // expanding tildes into absolute paths and adding the data subdir to each path here
                PathBuf::from(shellexpand::tilde(&p.to_string_lossy()).into_owned()).join("data")
            })
            .collect();

        WebScrapbookFiles { paths }
    }

    /// Collects all of the entries and sends them to the indexer.
    ///
    /// This method does this in a separate thread.
    pub(crate) fn collect_entries(&self, sender: IndexPathSender) {
        for dir in &self.paths {
            // we're limiting the depth since these entries are all stored in the same dir
            for result in WalkBuilder::new(dir).max_depth(Some(1)).build() {
                match result {
                    // TODO switch to the tracing crate
                    Err(e) => {
                        // TODO collect these errors so the user can see what is not being indexed properly
                        println!("could not open path: {}", e)
                    }

                    Ok(entry) => {
                        let web_scrapbook_index = entry.path().join("index.html");
                        if web_scrapbook_index.exists() && web_scrapbook_index.is_file() {
                            // TODO check the file type here
                            // if this is a file, send it to the fulltext index to check if it is already indexed and up to date
                            println!("sending path {}...", web_scrapbook_index.to_string_lossy());
                            sender
                                .send(IndexPath::WebScrapBookFile(web_scrapbook_index))
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
        println!("attempting to convert path to entry...");

        // TODO properly handle non UTF-8 file contents
        // TODO handle very large files efficiently, maybe switch to a streaming library
        let body = fs::read_to_string(&path).unwrap_or_default();

        // Extract title according to priority rules
        let title = Self::extract_title(&body);

        // convert the body to markdown before returning
        let doc = Document::from(body);
        // TODO ensure that any extraneous elements like styling and classes are not passed along
        // TODO run this through the readability library
        let markdown_body = doc.md(None);

        IndexEntry::new(
            WEB_SCRAPBOOK_FILES_SOURCE.to_owned(),
            path.to_string_lossy().to_string(),
            title,
            markdown_body.to_string(),
        )
    }

    fn extract_title(body: &str) -> String {
        // read the first line and pull the title from
        // <!DOCTYPE html><html lang="en" data-scrapbook-source="https://www.theguardian.com/business/2023/aug/28/phoenix-microchip-plant-biden-union-tsmc" data-scrapbook-create="20231215012632582" data-scrapbook-title="‘They would not listen to us’: inside Arizona’s troubled chip plant | Business | The Guardian">
        let title = body
            .lines()
            .next()
            .and_then(|l| {
                Document::fragment(l)
                    .select("html")
                    .attr("data-scrapbook-title")
                    .map(|a| a.to_string())
            })
            .unwrap_or("UNKNOWN_TITLE".to_owned());

        // TODO add fallback here to pull title from the page, don't forget to clean it up (squeeze and trim whitespace, make it one line)

        title
    }
}
