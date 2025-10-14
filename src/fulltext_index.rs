use ignore::Walk;
use std::{fs::create_dir_all, path::PathBuf};
use tantivy::{
    Index, TantivyError,
    directory::{ManagedDirectory, MmapDirectory},
    doc,
    schema::{STORED, Schema, TEXT},
};

use crate::{
    config::{Conf, PathList},
    retsyn_app::PROJECT_DIRS,
};

pub(crate) struct FulltextIndex {
    markdown_files: PathList,
    index: Index,
}

impl FulltextIndex {
    pub(crate) fn new(config: &Conf) -> Result<Self, TantivyError> {
        // setup the schema
        let mut schema_builder = Schema::builder();
        // the filename
        schema_builder.add_text_field("name", TEXT | STORED);
        // the title of the file
        schema_builder.add_text_field("title", TEXT | STORED);
        // the main text of the file
        schema_builder.add_text_field("body", TEXT);
        let schema = schema_builder.build();

        // create the index
        let index_path = PROJECT_DIRS.cache_dir().join("tantivy");
        create_dir_all(&index_path)?;
        println!("opening tantivy index in: {}", index_path.to_string_lossy());
        let index_dir = ManagedDirectory::wrap(Box::new(
            MmapDirectory::open(index_path)
                .expect("should be able to create the tantivy mmap directory"),
        ))
        .expect("should be able to create the tantivy managed directory");
        let index = Index::open_or_create(index_dir, schema)
            .expect("should be able to open or create the index");

        Ok(Self {
            markdown_files: config
                .markdown_files
                .iter()
                // expand tildes into absolute paths
                .map(|p| PathBuf::from(shellexpand::tilde(&p.to_string_lossy()).into_owned()))
                .collect(),
            index,
        })
    }

    pub(crate) fn update(&self) -> Result<(), TantivyError> {
        let mut index_writer = self
            .index
            .writer(50_000_000)
            .expect("should be able to obtain an index writer");

        let schema = self.index.schema();
        let title = schema.get_field("title").unwrap();
        let body = schema.get_field("body").unwrap();

        for dir in &self.markdown_files {
            for result in Walk::new(dir) {
                match result {
                    // TODO replace the
                    Ok(entry) => {
                        let title_text = entry.path().to_string_lossy();
                        match index_writer.add_document(doc!(title => *title_text, body => "body"))
                        {
                            Ok(_) => println!("{}", title_text),
                            Err(e) => println!("could not index document: {}", e),
                        }
                    }
                    Err(e) => println!("could not open path: {}", e),
                }
            }
        }

        index_writer.commit()?;

        Ok(())
    }
}
