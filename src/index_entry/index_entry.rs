use std::path::PathBuf;

pub(crate) trait IndexEntry {
    fn source(&self) -> &str;
    fn indexed_at(&self) -> time::OffsetDateTime;
    fn path(&self) -> &PathBuf;
    fn title(&self) -> String;
    fn body(&self) -> String;
}

pub(crate) enum IndexEntry {
    MarkdownFile(PathBuf),
}
