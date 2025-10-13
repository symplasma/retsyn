use crate::config::{Conf, PathList};

pub(crate) struct FulltextIndex {
    markdown_files: PathList,
}

impl FulltextIndex {
    pub(crate) fn new(config: &Conf) -> Self {
        Self {
            markdown_files: config.markdown_files.clone(),
        }
    }
}
