use std::path::PathBuf;

// AI! Create an `IndexEntry` trait with the following methods: source, indexed_at, path, title, body

pub(crate) enum IndexEntry {
    MarkdownFile(PathBuf),
}
