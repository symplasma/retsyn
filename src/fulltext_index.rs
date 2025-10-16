use atomicwrites::{AtomicFile, OverwriteBehavior::AllowOverwrite};
use ignore::Walk;
use std::{
    fs::{self, create_dir_all},
    io::Write,
    path::PathBuf,
    sync::LazyLock,
};
use tantivy::{
    DateTime, Index, IndexReader, IndexSettings, ReloadPolicy, TantivyDocument, TantivyError,
    collector::TopDocs,
    directory::{ManagedDirectory, MmapDirectory},
    query::QueryParser,
    schema::{DateOptions, INDEXED, STORED, Schema, TEXT},
    snippet::SnippetGenerator,
};
use time::OffsetDateTime;

use crate::{
    config::{Conf, PathList},
    retsyn_app::PROJECT_DIRS,
    search_result::SearchResult,
};

pub(crate) static INDEXING_EPOCH_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PROJECT_DIRS
        .cache_dir()
        .to_path_buf()
        .join("last_indexing_epoch.txt")
});

const SOURCE: &str = "source";
const INDEXED_AT: &str = "indexed_at";
const PATH: &str = "path";
const TITLE: &str = "title";
const BODY: &str = "body";

pub(crate) struct FulltextIndex {
    last_indexing_epoch: Option<OffsetDateTime>,
    markdown_files: PathList,
    index: Index,
    reader: IndexReader,
}

impl FulltextIndex {
    pub(crate) fn new(config: &Conf) -> Result<Self, TantivyError> {
        // setup the schema
        let mut schema_builder = Schema::builder();
        // add the source, the module that discovered this file
        schema_builder.add_text_field(SOURCE, TEXT | STORED);
        // add the indexed at field
        let date_opts = DateOptions::from(INDEXED)
            .set_stored()
            .set_fast()
            .set_precision(tantivy::schema::DateTimePrecision::Seconds);
        schema_builder.add_date_field(INDEXED_AT, date_opts);
        // the filename
        schema_builder.add_text_field(PATH, TEXT | STORED);
        // the title of the file
        schema_builder.add_text_field(TITLE, TEXT | STORED);
        // the main text of the file
        schema_builder.add_text_field(BODY, TEXT | STORED);
        let schema = schema_builder.build();

        // create the index
        let index_path = PROJECT_DIRS.cache_dir().join("tantivy");
        create_dir_all(&index_path)?;
        println!(
            "tantivy index directory is: {}",
            index_path.to_string_lossy()
        );
        let index_dir = ManagedDirectory::wrap(Box::new(
            MmapDirectory::open(index_path)
                .expect("should be able to create the tantivy mmap directory"),
        ))
        .expect("should be able to create the tantivy managed directory");

        // attempt to read the last indexing epoch
        let last_indexing_epoch = match INDEXING_EPOCH_PATH.exists() {
            true => match fs::read_to_string(INDEXING_EPOCH_PATH.as_path()) {
                Ok(epoch_file) => match epoch_file.parse::<i64>() {
                    Ok(e) => match OffsetDateTime::from_unix_timestamp(e) {
                        Ok(dt) => Some(dt),
                        Err(_) => None,
                    },
                    Err(_) => None,
                },
                Err(_) => None,
            },
            false => None,
        };

        // if the indexing_epoch_file exists `open_or_create` the existing index, otherwise `create` a new one
        let index = if last_indexing_epoch.is_some() {
            println!("opening or creating tantivy index");
            // calling open or create just in case the epoch file exists but the actual index was deleted
            Index::open_or_create(index_dir, schema)?
        } else {
            println!("creating new tantivy index");
            Index::create(index_dir, schema, IndexSettings::default())?
        };

        // create the reader here
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        Ok(Self {
            last_indexing_epoch,
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

    /// Updates the fulltext index
    pub(crate) fn update(&self) -> Result<(), TantivyError> {
        let mut index_writer = self
            .index
            .writer(50_000_000)
            .expect("should be able to obtain an index writer");

        let schema = self.index.schema();
        let source = schema.get_field(SOURCE).unwrap();
        let indexed_at = schema.get_field(INDEXED_AT).unwrap();
        let path = schema.get_field(PATH).unwrap();
        let title = schema.get_field(TITLE).unwrap();
        let body = schema.get_field(BODY).unwrap();

        let source_str = "markdown_files";
        let indexing_date_time = OffsetDateTime::now_utc();
        // this seems like overly complicated type juggling
        let indexed_at_time =
            DateTime::from_primitive(DateTime::from_utc(indexing_date_time).into_primitive());

        // setting up the indexing epoch
        let indexing_epoch_file = AtomicFile::new(&*INDEXING_EPOCH_PATH, AllowOverwrite);
        let indexing_epoch = indexing_date_time.unix_timestamp();

        for dir in &self.markdown_files {
            for result in Walk::new(dir) {
                match result {
                    // TODO replace the
                    Ok(entry) => {
                        if entry.file_type().map(|e| e.is_file()).unwrap_or(false) {
                            // the file path on disk
                            // TODO we'll need to modify this or add a volume identifier if we index from more than one host
                            let entry_path = entry.path().to_string_lossy();
                            // TODO set title here based on file type and index source
                            let title_text = entry.path().to_string_lossy();
                            // TODO properly handle non UTF-8 file contents
                            let body_text = fs::read_to_string(entry.path()).unwrap_or_default();

                            // we were using the `doc!()` macro, but it doesn't seem to play well with date fields
                            let mut tantivy_doc = TantivyDocument::default();
                            tantivy_doc.add_text(source, source_str);
                            tantivy_doc.add_date(indexed_at, indexed_at_time);
                            tantivy_doc.add_text(path, entry_path.clone());
                            tantivy_doc.add_text(title, title_text);
                            tantivy_doc.add_text(body, body_text);

                            // add the document to the index
                            match index_writer.add_document(tantivy_doc) {
                                Ok(_) => println!("{}", &entry_path),
                                // TODO switch to the tracing crate
                                Err(e) => {
                                    println!("could not index document: {}: {}", entry_path, e)
                                }
                            }
                        }
                    }
                    // TODO switch to the tracing crate
                    Err(e) => println!("could not open path: {}", e),
                }
            }
        }

        // commit the changes so that searchers can see the changes
        index_writer.commit()?;

        // write the epoch of the last indexing to use for incremental updates
        println!(
            "writing indexing epoch {} to: {}",
            indexing_epoch,
            INDEXING_EPOCH_PATH.to_string_lossy()
        );
        indexing_epoch_file
            .write(|f| f.write_all(indexing_epoch.to_string().as_bytes()))
            .expect("should be able to write to the indexing epoch file");

        Ok(())
    }

    pub(crate) fn search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>, TantivyError> {
        let searcher = self.reader.searcher();

        let schema = self.index.schema();
        let source = schema.get_field(SOURCE).unwrap();
        let indexed_at = schema.get_field(INDEXED_AT).unwrap();
        let path = schema.get_field(PATH).unwrap();
        let title = schema.get_field(TITLE).unwrap();
        let body = schema.get_field(BODY).unwrap();

        let query_parser = QueryParser::for_index(&self.index, vec![title, body]);
        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let snippet_generator = SnippetGenerator::create(&searcher, &query, body)?;

        let mut documents: Vec<SearchResult> = Vec::default();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
            let snippet = snippet_generator.snippet_from_doc(&retrieved_doc);
            documents.push(SearchResult::new(
                source,
                indexed_at,
                path,
                title,
                retrieved_doc,
                snippet,
            ));
        }

        Ok(documents)
    }
}
