use std::{
    fmt,
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, Sender},
};
use tantivy::DateTime;
use time::OffsetDateTime;

/// A channel to send IndexEtries to other threads
pub(crate) type IndexEntrySender = Sender<IndexEntry>;

/// A channel to receive IndexEtries from other threads
pub(crate) type IndexEntryReceiver = Receiver<IndexEntry>;

pub(crate) type IndexPathSender = Sender<IndexPath>;
pub(crate) type IndexPathReceiver = Receiver<IndexPath>;

pub(crate) enum IndexPath {
    MarkdownFile(PathBuf),
    WebScrapBookFile(PathBuf),
    AichatSessionFile(PathBuf),
}

impl fmt::Display for IndexPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexPath::MarkdownFile(path) => write!(f, "MarkdownFile({})", path.display()),
            IndexPath::WebScrapBookFile(path) => write!(f, "WebScrapBookFile({})", path.display()),
            IndexPath::AichatSessionFile(path) => write!(f, "AichatSessionFile({})", path.display()),
        }
    }
}

impl IndexPath {
    pub(crate) fn path(&self) -> &Path {
        match self {
            IndexPath::MarkdownFile(path_buf) => path_buf,
            IndexPath::WebScrapBookFile(path_buf) => path_buf,
            IndexPath::AichatSessionFile(path_buf) => path_buf,
        }
    }
}

/// An index entry represents the data and metadata from an item that needs to be added to the index.
///
/// IndexEntries are sent via channels from collectors to the indexer. This allows us to perform collection and indexing in separate threads to keep everything performant.
pub(crate) struct IndexEntry {
    source: String,
    indexed_at: DateTime,
    path: String,
    title: String,
    body: String,
}

impl IndexEntry {
    pub(crate) fn new(source: String, path: String, title: String, body: String) -> Self {
        IndexEntry {
            source,
            // TODO consider adding making this the same time for a given indexing session
            indexed_at: DateTime::from_utc(OffsetDateTime::now_utc()),
            path,
            title,
            body,
        }
    }

    pub(crate) fn source(&self) -> &str {
        &self.source
    }

    pub(crate) fn indexed_at(&self) -> &DateTime {
        &self.indexed_at
    }

    pub(crate) fn path(&self) -> &str {
        &self.path
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn body(&self) -> &str {
        &self.body
    }
}
