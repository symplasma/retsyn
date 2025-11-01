use crate::fulltext_index::{IndexStatus, SearchResultsAndErrors};

pub(crate) enum IndexResults {
    // TODO change to a proper error type
    Error(String),
    Status(IndexStatus),
    SearchResults {
        opstamp: tantivy::Opstamp,
        results: SearchResultsAndErrors,
    },
}
