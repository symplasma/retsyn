use atomicwrites::{AtomicFile, OverwriteBehavior::AllowOverwrite};
use std::{
    fs::{self, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
    sync::{
        LazyLock,
        mpsc::{Receiver, Sender, channel},
    },
    thread::{self, spawn},
    time::{Duration, Instant},
};
use tantivy::{
    Index, IndexReader, IndexSettings, IndexWriter, ReloadPolicy, TantivyDocument, TantivyError,
    Term,
    collector::{Count, TopDocs},
    directory::{ManagedDirectory, MmapDirectory},
    query::{QueryParser, QueryParserError, TermQuery},
    schema::{
        DateOptions, Field as TantivyField, INDEXED, IndexRecordOption, STORED, Schema, TEXT,
        TextFieldIndexing, TextOptions,
    },
    snippet::SnippetGenerator,
};
use time::OffsetDateTime;
use tracing::{debug, info, warn};

use crate::{
    collectors::{
        aichat_session_files::AichatSessionFiles, markdown_files::MarkdownFiles,
        web_scrapbook_files::WebScrapbookFiles,
    },
    config::Conf,
    index_entry::index_entry::{
        IndexEntry, IndexEntrySender, IndexPath, IndexPathReceiver, IndexPathSender,
    },
    messages::{index_request::IndexRequest, index_results::IndexResults},
    retsyn_app::PROJECT_DIRS,
    search_result::SearchResult,
};

pub(crate) static INDEX_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PROJECT_DIRS.cache_dir().join("tantivy"));

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

#[derive(Debug)]
pub(crate) enum IndexStatus {
    Initializing,
    CollectingPaths,
    FilteringPaths,
    UpdatingIndex,
    CommittingChanges,
    UpToDate,
}

pub struct FulltextIndex {
    // status: IndexStatus,
    config: Conf,
    last_indexing_epoch: Option<OffsetDateTime>,
    index: Index,
    reader: IndexReader,
    writer: IndexWriter,
    request_receiver: Receiver<IndexRequest>,
    results_sender: Sender<IndexResults>,
    pub(crate) source_field: TantivyField,
    pub(crate) indexed_at_field: TantivyField,
    pub(crate) path_field: TantivyField,
    pub(crate) title_field: TantivyField,
    pub(crate) body_field: TantivyField,
}

pub(crate) type SearchResultsAndErrors =
    Result<(Vec<SearchResult>, Vec<QueryParserError>), TantivyError>;

fn last_indexing_epoch() -> Option<OffsetDateTime> {
    match INDEXING_EPOCH_PATH.exists() {
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
    }
}

fn tantivy_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    // the filepath, we're setting up custom options so we can reliably search paths
    let text_field_indexing = TextFieldIndexing::default()
        // we do NOT want the field tokenized
        .set_tokenizer("raw")
        .set_index_option(IndexRecordOption::Basic);
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();
    schema_builder.add_text_field(PATH, text_options);

    // add the source, the module that discovered this file
    schema_builder.add_text_field(SOURCE, TEXT | STORED);
    // add the indexed at field
    let date_opts = DateOptions::from(INDEXED)
        .set_stored()
        .set_fast()
        .set_precision(tantivy::schema::DateTimePrecision::Seconds);
    schema_builder.add_date_field(INDEXED_AT, date_opts);
    // the title of the file
    schema_builder.add_text_field(TITLE, TEXT | STORED);
    // the main text of the file
    schema_builder.add_text_field(BODY, TEXT | STORED);
    let schema = schema_builder.build();
    schema
}

impl FulltextIndex {
    fn send_status(&self, status: IndexStatus) {
        self.results_sender
            .send(IndexResults::Status(status))
            .expect("should be able to send status");
    }

    pub(crate) fn new(
        config: Conf,
        request_receiver: Receiver<IndexRequest>,
        results_sender: Sender<IndexResults>,
    ) -> Result<Self, TantivyError> {
        // setup the schema
        let schema = tantivy_schema();

        // create the index
        create_dir_all(&*INDEX_PATH)?;
        info!(
            "tantivy index directory is: {}",
            INDEX_PATH.to_string_lossy()
        );
        let index_dir = ManagedDirectory::wrap(Box::new(
            MmapDirectory::open(INDEX_PATH.as_path())
                .expect("should be able to create the tantivy mmap directory"),
        ))
        .expect("should be able to create the tantivy managed directory");

        // attempt to read the last indexing epoch
        let last_indexing_epoch = last_indexing_epoch();

        // if the indexing_epoch_file exists `open_or_create` the existing index, otherwise `create` a new one
        let index = if last_indexing_epoch.is_some() {
            debug!("opening or creating tantivy index");
            // calling open or create just in case the epoch file exists but the actual index was deleted
            Index::open_or_create(index_dir, schema.clone())?
        } else {
            debug!("creating new tantivy index");
            Index::create(index_dir, schema.clone(), IndexSettings::default())?
        };

        // create the reader here
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        let writer = index
            .writer(50_000_000)
            .expect("should be able to obtain an index writer");

        let source_field = schema.get_field(SOURCE).unwrap();
        let indexed_at_field = schema.get_field(INDEXED_AT).unwrap();
        let path_field = schema.get_field(PATH).unwrap();
        let title_field = schema.get_field(TITLE).unwrap();
        let body_field = schema.get_field(BODY).unwrap();

        Ok(Self {
            // status: IndexStatus::Initializing,
            config,
            last_indexing_epoch,
            index,
            reader,
            writer,
            request_receiver,
            results_sender,
            source_field,
            indexed_at_field,
            path_field,
            title_field,
            body_field,
        })
    }

    /// Clear the search index by removing the index directory and epoch file
    pub fn clear_index() -> std::io::Result<()> {
        let index_path = PROJECT_DIRS.cache_dir().join("tantivy");

        // Remove the index directory if it exists
        if index_path.exists() {
            fs::remove_dir_all(&index_path)?;
            info!("Removed index directory: {}", index_path.display());
        }

        // Remove the indexing epoch file if it exists
        if INDEXING_EPOCH_PATH.exists() {
            fs::remove_file(&*INDEXING_EPOCH_PATH)?;
            info!(
                "Removed indexing epoch file: {}",
                INDEXING_EPOCH_PATH.display()
            );
        }

        Ok(())
    }

    /// Delete the given entry from the index by its path
    ///
    /// This takes an `&str` since the IndexEntry has not been constructed yet when this is called.
    pub(crate) fn delete_entry(&self, entry_path: &str) {
        let doc_path_term = Term::from_field_text(self.path_field, entry_path);
        self.writer.delete_term(doc_path_term);
    }

    /// Loop over the paths that the collectors have found, see if they are indexed and up to date. If so pass them on to the loader.
    pub(crate) fn filter_paths_to_update(
        &mut self,
        path_receiver: IndexPathReceiver,
        path_converter_sender: IndexPathSender,
    ) {
        self.send_status(IndexStatus::FilteringPaths);
        for index_path in path_receiver {
            // the file path on disk
            // TODO we'll need to modify this or add a volume identifier if we index from more than one host
            let path = index_path.path();
            let path_str = path.to_string_lossy();
            debug!("checking if {} needs to be updated...", path_str);

            let mut entry_up_to_date = false;

            // see if the document is already present in the index
            if self.file_is_indexed(path) {
                debug!("found document in index: {}", &path_str);
                // if the last_indexing_epoch is Some and the file's last update time is later than it, then delete the entry from the index by path
                entry_up_to_date = self.entry_up_to_date(&path);

                // only check the update time if the item is already in the database
                if !entry_up_to_date {
                    self.delete_entry(&path_str)
                }
            }

            // if the entry does not need an update, continue with the next item
            if entry_up_to_date {
                continue;
            };

            path_converter_sender
                .send(index_path)
                .expect("should be able to send path to converter");
        }

        drop(path_converter_sender);
    }

    pub(crate) fn start_collectors(&mut self) -> Receiver<IndexEntry> {
        self.send_status(IndexStatus::CollectingPaths);

        let (path_sender, path_receiver) = channel();
        let (path_converter_sender, path_converter_receiver) = channel();
        let (entry_sender, entry_receiver) = channel();

        // spawn loader channels here
        let aichat_session_files = AichatSessionFiles::new(&self.config.aichat_session_files);
        let markdown_files = MarkdownFiles::new(&self.config.markdown_files);
        let web_scrapbook_files = WebScrapbookFiles::new(&self.config.web_scrapbook_files);

        // start collecting various entries in separate threads here
        info!("spawning aichat session file entry collection...");
        let aichat_session_path_sender = path_sender.clone();
        spawn(move || {
            aichat_session_files.collect_entries(aichat_session_path_sender);
        });

        info!("spawning markdown file entry collection...");
        let markdown_path_sender = path_sender.clone();
        spawn(move || {
            markdown_files.collect_entries(markdown_path_sender);
        });

        info!("spawning web scrapbook file entry collection...");
        let web_scrapbook_path_sender = path_sender.clone();
        spawn(move || {
            web_scrapbook_files.collect_entries(web_scrapbook_path_sender);
        });

        // dropping the original path sender so we don't hang the program waiting for more paths
        drop(path_sender);

        // this is basically happening synchronously here since it depends on the tantivy index
        // might need to wrap the indexes in Arcs and possibly Mutexes to make this asynchronous
        info!("filtering paths...");
        self.filter_paths_to_update(path_receiver, path_converter_sender);

        info!("spawning path to entry converter...");
        spawn(move || Self::convert_paths_to_entries(path_converter_receiver, entry_sender));

        entry_receiver
    }

    /// Updates the fulltext index by reading the IndexEntries from the receiver
    pub(crate) fn update(
        &mut self,
        entry_receiver: Receiver<IndexEntry>,
    ) -> Result<(), TantivyError> {
        info!("updating the fulltext index...");

        // setting up the indexing epoch
        let indexing_epoch_file = AtomicFile::new(&*INDEXING_EPOCH_PATH, AllowOverwrite);
        let indexing_epoch = OffsetDateTime::now_utc().unix_timestamp();

        self.send_status(IndexStatus::UpdatingIndex);

        // loop through all of the IndexEntries on the receiver and add them to the index
        info!("starting entry indexing loop...");
        let indexing_start = Instant::now();
        loop {
            // TODO consider switching to crossbeam's select macro
            // TODO count the entries received or the time taken so we can commit in batches

            if indexing_start.elapsed().as_secs() > 2 {
                self.send_status(IndexStatus::CommittingChanges);

                // commit the changes so that searchers can see the changes
                info!("committing changes to fulltext index...");
                self.writer
                    .commit()
                    .expect("should be able to commit index updates");

                self.send_status(IndexStatus::UpdatingIndex);
            }

            // check for new entries in need of indexing
            match entry_receiver.try_recv() {
                Ok(entry) => self.update_entry(&entry),
                Err(e) => match e {
                    std::sync::mpsc::TryRecvError::Empty => {
                        // check for new search requests
                        if let Ok(request) = self.request_receiver.try_recv() {
                            self.search(request)
                        } else {
                            thread::sleep(Duration::from_millis(20));
                        }
                        continue;
                    }
                    std::sync::mpsc::TryRecvError::Disconnected => break,
                },
            }
        }

        self.send_status(IndexStatus::CommittingChanges);

        // commit the changes so that searchers can see the changes
        info!("committing changes to fulltext index...");
        self.writer
            .commit()
            .expect("should be able to commit index updates");

        // write the epoch of the last indexing to use for incremental updates
        info!(
            "writing indexing epoch {} to: {}",
            indexing_epoch,
            INDEXING_EPOCH_PATH.to_string_lossy()
        );
        indexing_epoch_file
            .write(|f| f.write_all(indexing_epoch.to_string().as_bytes()))
            .expect("should be able to write to the indexing epoch file");

        self.send_status(IndexStatus::UpToDate);

        Ok(())
    }

    pub(crate) fn update_search_results(&self) {
        while let Ok(request) = self.request_receiver.recv() {
            self.search(request)
        }
    }

    fn update_entry(&self, entry: &IndexEntry) {
        // we were using the `doc!()` macro, but it doesn't seem to play well with date fields
        let mut tantivy_doc = TantivyDocument::default();
        tantivy_doc.add_text(self.source_field, entry.source());
        tantivy_doc.add_date(self.indexed_at_field, *entry.indexed_at());
        tantivy_doc.add_text(self.path_field, entry.path());
        tantivy_doc.add_text(self.title_field, entry.title());
        tantivy_doc.add_text(self.body_field, entry.body());

        // add the document to the index
        match self.writer.add_document(tantivy_doc) {
            Ok(_) => info!("adding document to index: {}", &entry.path()),
            // TODO switch to the tracing crate
            Err(e) => {
                warn!("could not index document: {}: {}", &entry.path(), e)
            }
        }
    }

    fn convert_paths_to_entries(path_receiver: IndexPathReceiver, entry_sender: IndexEntrySender) {
        for index_path in path_receiver {
            debug!("attempting to convert {} to entry...", index_path);
            let new_entry = match index_path {
                IndexPath::MarkdownFile(path_buf) => {
                    MarkdownFiles::convert_path_to_entry(&path_buf)
                }
                IndexPath::WebScrapBookFile(path_buf) => {
                    WebScrapbookFiles::convert_path_to_entry(&path_buf)
                }
                IndexPath::AichatSessionFile(path_buf) => {
                    AichatSessionFiles::convert_path_to_entry(&path_buf)
                }
            };

            // TODO maybe send these in a Box or Arc to reduce memory allocations
            entry_sender
                .send(new_entry)
                .expect("should be able to send new entry to indexer");
        }

        drop(entry_sender);
    }

    pub(crate) fn search(&self, request: IndexRequest) {
        let searcher = self.reader.searcher();
        let title = self.title_field;
        let body = self.body_field;
        let default_fields = vec![title, body];

        // setup the query here
        let mut query_parser = QueryParser::for_index(&self.index, default_fields.clone());

        if request.query_conjunction {
            query_parser.set_conjunction_by_default();
        }

        // set fields fuzzy here
        // TODO add advanced search config where individual field can have its fuzziness set independently
        if request.fuzziness > 0 {
            for field in &default_fields {
                query_parser.set_field_fuzzy(*field, true, request.fuzziness, true);
            }
        }

        // parse the query here
        let (query, query_errors) = if request.lenient {
            query_parser.parse_query_lenient(&request.query)
        } else {
            match query_parser.parse_query(&request.query) {
                Ok(query) => (query, vec![]),
                // if we have an error in non-lenient parsing, return with no results
                Err(error) => {
                    self.results_sender
                        .send(IndexResults::SearchResults {
                            request_id: request.request_id,
                            // TODO return the actual opstamp
                            opstamp: 0,
                            results: Ok((vec![], vec![error])),
                        })
                        .expect("should be able to send results");

                    return;
                }
            }
        };

        // perform the search
        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(request.limit))
            .unwrap();

        // create a snippet generator so we can draw snippets with highlights
        let snippet_generator = SnippetGenerator::create(&searcher, &query, body).unwrap();

        let mut documents: Vec<SearchResult> = Vec::default();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();
            let snippet = snippet_generator.snippet_from_doc(&retrieved_doc);
            documents.push(SearchResult::new(self, retrieved_doc, snippet));
        }

        let num_hits = documents.len();
        match self.results_sender.send(IndexResults::SearchResults {
            request_id: request.request_id,
            // TODO return the actual opstamp
            opstamp: 0,
            results: Ok((documents, query_errors)),
        }) {
            Ok(_) => info!(
                "sending {} results for query {}: {}",
                num_hits, request.request_id, request.query
            ),
            Err(e) => warn!("should be able to send results: {}", e),
        };
    }

    pub(crate) fn file_is_indexed(&self, path: &Path) -> bool {
        let schema = self.index.schema();
        let path_field = schema.get_field(PATH).unwrap();

        // TODO may want to call a custom method so this doesn't get out of sync between indexing and our search here
        let query_text = &path.to_string_lossy();
        let query = TermQuery::new(
            Term::from_field_text(path_field, query_text),
            IndexRecordOption::Basic,
        );

        let searcher = self.reader.searcher();

        // see if the document is already present in the index
        match searcher.search(&query, &Count) {
            Err(e) => {
                warn!("error searching for document: {}", e);
                false
            }
            Ok(count) => {
                // println!("found in index {} times: {}", count, path.to_string_lossy());
                if count > 0 {
                    true
                } else {
                    // println!("document not found in index: {}", file_path);
                    false
                }
            }
        }
    }

    pub(crate) fn entry_up_to_date(&self, entry: &Path) -> bool {
        match self.last_indexing_epoch {
            None => {
                info!("last indexing epoch is not set");
                true
            }
            Some(last_indexing_epoch) => match entry.metadata() {
                Err(e) => {
                    warn!("could not get metadata: {}", e);
                    true
                }
                Ok(metadata) => match metadata.modified() {
                    Err(e) => {
                        warn!("could not get modification date: {}", e);
                        true
                    }
                    Ok(file_modified) => {
                        if file_modified > last_indexing_epoch {
                            // println!(
                            //     "deleting file from index: {}",
                            //     entry.path().to_string_lossy()
                            // );
                            false
                        } else {
                            true
                        }
                    }
                },
            },
        }
    }
}
