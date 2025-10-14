use ignore::Walk;
use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
};
use tantivy::{
    Index, IndexReader, ReloadPolicy, TantivyDocument, TantivyError,
    collector::TopDocs,
    directory::{ManagedDirectory, MmapDirectory},
    doc,
    query::QueryParser,
    schema::{STORED, Schema, TEXT},
    snippet::SnippetGenerator,
};

use crate::{
    config::{Conf, PathList},
    retsyn_app::PROJECT_DIRS,
    search_result::SearchResult,
};

pub(crate) struct FulltextIndex {
    markdown_files: PathList,
    index: Index,
    reader: IndexReader,
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
        schema_builder.add_text_field("body", TEXT | STORED);
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

        // create the reader here
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        Ok(Self {
            markdown_files: config
                .markdown_files
                .iter()
                // expand tildes into absolute paths
                .map(|p| PathBuf::from(shellexpand::tilde(&p.to_string_lossy()).into_owned()))
                .collect(),
            index,
            reader,
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
                        if entry.file_type().map(|e| e.is_file()).unwrap_or(false) {
                            let title_text = entry.path().to_string_lossy();
                            // TODO properly handle non UTF-8 file contents
                            let body_text = fs::read_to_string(entry.path()).unwrap_or_default();
                            match index_writer
                                .add_document(doc!(title => *title_text, body => body_text))
                            {
                                Ok(_) => println!("{}", title_text),
                                Err(e) => println!("could not index document: {}", e),
                            }
                        }
                    }
                    Err(e) => println!("could not open path: {}", e),
                }
            }
        }

        index_writer.commit()?;

        Ok(())
    }

    pub(crate) fn search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>, TantivyError> {
        let searcher = self.reader.searcher();

        let schema = self.index.schema();
        let title = schema.get_field("title").unwrap();
        let body = schema.get_field("body").unwrap();

        let query_parser = QueryParser::for_index(&self.index, vec![title, body]);
        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let snippet_generator = SnippetGenerator::create(&searcher, &query, body)?;

        let mut documents: Vec<SearchResult> = Vec::default();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
            let snippet = snippet_generator.snippet_from_doc(&retrieved_doc);
            documents.push(SearchResult::new(title, retrieved_doc, snippet));
        }

        Ok(documents)
    }
}
