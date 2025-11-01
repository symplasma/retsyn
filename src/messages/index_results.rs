use crate::fulltext_index::{IndexStatus, SearchResultsAndErrors};

pub(crate) enum IndexResults {
    // TODO change to a proper error type
    Error(String),
    Status(IndexStatus),
    SearchResults {
        request_id: usize,
        opstamp: tantivy::Opstamp,
        results: SearchResultsAndErrors,
    },
}
