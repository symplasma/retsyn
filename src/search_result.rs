use tantivy::{
    TantivyDocument,
    schema::{Field, Value as _},
};

#[derive(Debug)]
pub(crate) struct SearchResult {
    title: String,
    snippet: String,
    tantivy_doc: TantivyDocument,
}

impl SearchResult {
    pub(crate) fn new(title: Field, doc: TantivyDocument) -> Self {
        Self {
            title: doc
                .get_first(title)
                .map(|t| t.as_str())
                .flatten()
                .unwrap_or_default()
                .to_owned(),
            snippet: "TBD".to_owned(),
            tantivy_doc: doc,
        }
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }
}
