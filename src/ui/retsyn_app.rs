use std::{
    process::exit,
    sync::{
        LazyLock,
        mpsc::{Receiver, Sender, channel},
    },
    thread::spawn,
    time::{Duration, Instant},
    vec,
};

use confique::Config;
use directories::ProjectDirs;
use eframe::CreationContext;
use egui::Context;
use tracing::{error, info, warn};

use crate::{
    config::Conf,
    invocations::{
        invocation::{Action, Invocation},
        invocation_list::InvocationList,
    },
    messages::{index_request::IndexRequest, index_results::IndexResults},
    model::fulltext_index::{FulltextIndex, IndexStatus, SearchResultsAndErrors},
    model::search_result::SearchResult,
};

const INTERFRAME_MILLIS: u64 = 16;
const DEBOUNCE_DURATION: Duration = Duration::from_millis(150);

pub(crate) static PROJECT_DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("org", "symplasma", "retsyn").expect("should be able to create project dir")
});

pub struct RetsynApp {
    egui_ctx: Context,
    pub(crate) search_text: String,
    last_search_text: String,
    last_request_id: usize,
    last_response_id: usize,
    pub(crate) index_status: IndexStatus,
    pub(crate) matched_items: SearchResultsAndErrors,
    pub(crate) selected_index: Option<usize>,
    pub(crate) last_input_time: Option<Instant>,
    debounce_duration: Duration,
    pub(crate) recent_queries: InvocationList,
    pub(crate) invocations: InvocationList,
    pub(crate) scroll_to_selected: bool,
    dark_mode: bool,
    pub(crate) show_snippets: bool,
    pub(crate) show_preview: bool,
    pub(crate) show_help: bool,
    pub(crate) show_config: bool,
    pub(crate) config: Conf,
    pub(crate) config_markdown_files: Vec<String>,
    pub(crate) limit_results: usize,
    pub(crate) lenient: bool,
    pub(crate) query_conjunction: bool,
    pub(crate) fuzziness: u8,
    request_sender: Sender<IndexRequest>,
    results_receiver: Receiver<IndexResults>,
    last_repaint_request: Instant,
}

impl RetsynApp {
    pub fn new(cc: &CreationContext) -> Self {
        let egui_ctx = cc.egui_ctx.clone();

        let config_file = PROJECT_DIRS.config_dir().to_path_buf().join("retsyn.toml");
        let config_exists = Conf::config_exists();

        let config = match Conf::builder().env().file(&config_file).load() {
            Ok(config) => config,
            Err(_) => {
                // If config doesn't exist, create a default one
                Conf::builder().env().load().unwrap_or_else(|e| {
                    // If even that fails, use hardcoded defaults
                    error!(
                        "could not load config from {}: {}",
                        config_file.to_string_lossy(),
                        e
                    );

                    // not sure what this println was doing, maybe it was supposed to write a default config file
                    // println!("{}", toml::template::<Conf>(FormatOptions::default()));

                    // TODO notify the user that an error has occurred

                    // TODO unify errors and return error rather than exiting here
                    exit(0)
                })
            }
        };

        // Convert PathBuf to String for editing
        let config_markdown_files: Vec<String> = config
            .markdown_files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        if !config_exists {
            // TODO unify errors and return an error instead
            exit(1);
        }

        let (request_sender, request_receiver) = channel();
        let (results_sender, results_receiver) = channel();

        let fulltext_index_config = config.clone();
        spawn(move || {
            let mut index =
                FulltextIndex::new(fulltext_index_config, request_receiver, results_sender)
                    .unwrap();
            let entry_receiver = index.start_collectors();
            index.update(entry_receiver).unwrap();
            index.update_search_results();
        });

        // TODO pull this from config
        let dark_mode = false;
        // Set theme based on dark_mode toggle
        egui_ctx.set_visuals(if dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });

        Self {
            egui_ctx,
            search_text: String::new(),
            last_search_text: String::new(),
            last_request_id: 0,
            last_response_id: 0,
            index_status: IndexStatus::Initializing,
            matched_items: Ok((vec![], vec![])),
            selected_index: None,
            last_input_time: None,
            debounce_duration: DEBOUNCE_DURATION,
            recent_queries: InvocationList::load_from_cache().unwrap_or_default(),
            invocations: Default::default(),
            scroll_to_selected: false,
            dark_mode,
            show_snippets: true,
            show_preview: true,
            show_help: false,
            show_config: !config_exists,
            config,
            config_markdown_files,
            limit_results: 50,
            lenient: true,
            query_conjunction: true,
            fuzziness: 0,
            request_sender,
            results_receiver,
            last_repaint_request: Instant::now(),
        }
    }

    /// Returns the currently selected item as a reference
    ///
    /// This is useful to render the preview if it is shown.
    pub(crate) fn selected_item(&self) -> Option<&SearchResult> {
        match self.selected_index {
            Some(selected_index) => match &self.matched_items {
                Ok((items, _errors)) => items.get(selected_index),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub(crate) fn search(
        &mut self,
        query: &str,
        limit: usize,
        lenient: bool,
        query_conjunction: bool,
        fuzziness: u8,
    ) {
        self.last_request_id = self.last_request_id.saturating_add(1);
        match self.request_sender.send(IndexRequest {
            request_id: self.last_request_id,
            query: query.to_string(),
            limit,
            lenient,
            query_conjunction,
            fuzziness,
        }) {
            Ok(_) => info!(
                "sent search request {} for: {}",
                self.last_request_id, query
            ),
            Err(e) => warn!(
                "could not send search request {} for: {}: {}",
                self.last_request_id, query, e
            ),
        }
    }

    pub(crate) fn clear_search(&mut self) {
        self.search_text.clear();
        self.matched_items = Ok((vec![], vec![]));
        self.selected_index = None;
    }

    fn retrieve_results(&mut self) {
        let mut results_received: usize = 0;

        // we're looping here to soak up all pending results
        // if this becomes a performance issue we can bail early
        for index_results in self.results_receiver.try_iter() {
            results_received += 1;
            match index_results {
                IndexResults::Error(_) => todo!(),
                IndexResults::Status(index_status) => self.index_status = index_status,
                IndexResults::SearchResults {
                    request_id,
                    // TODO check the opstamp to see if there has been an index commit since our last search
                    opstamp,
                    results,
                } => {
                    self.last_response_id = request_id;
                    self.matched_items = results
                }
            }
        }

        // TODO determine if this is the selection preservation behavior that we want
        self.selected_index = Some(
            self.matched_items
                .as_ref()
                .and_then(|(m, _errors)| Ok(self.selected_index.min(Some(m.len()))))
                .unwrap_or_default()
                .unwrap_or_default(),
        );

        if results_received > 0
            || !matches!(self.index_status, IndexStatus::UpToDate)
            || self.last_request_id > self.last_response_id
        {
            // request screen repaint on changes
            self.egui_ctx.request_repaint();
            // self.egui_context.request_repaint_after(Duration::from_millis(INTERFRAME_MILLIS));
        }
    }

    pub(crate) fn update_search(&mut self) {
        if self.search_text.is_empty() {
            self.matched_items = Ok((vec![], vec![]));
            self.selected_index = None;
        } else {
            self.search(
                &self.search_text.clone(),
                self.limit_results,
                self.lenient,
                self.query_conjunction,
                self.fuzziness,
            );
        }

        // NOTE: we request a repaint in `retrieve_results`
        self.retrieve_results();
        self.last_search_text = self.search_text.clone();
    }

    pub(crate) fn open_item(&mut self, index: usize, reveal: bool) {
        if let Ok((matched_items, _errors)) = &self.matched_items {
            if index < matched_items.len() {
                let item = &matched_items[index];
                if reveal {
                    item.reveal();
                    // TODO add action to invocations
                    self.invocations.add_invocation_by_item(
                        Action::Reveal,
                        &self.search_text,
                        item,
                    );
                } else {
                    self.invocations
                        .add_invocation_by_item(Action::Open, &self.search_text, item);
                    item.open();
                }
            }
        }
    }

    /// Save invocations to CSV file
    fn save_invocations(&self) {
        let now = time::OffsetDateTime::now_utc();
        let cache_file = Invocation::cache_file(now);

        match Invocation::append_invocations_to_csv(&self.invocations, &cache_file) {
            Ok(()) => {
                info!("Successfully saved invocations to {}", cache_file.display());
            }
            Err(e) => {
                warn!(
                    "Failed to save invocations to {}: {}",
                    cache_file.display(),
                    e
                );
            }
        }
    }
}

impl Drop for RetsynApp {
    fn drop(&mut self) {
        info!("RetsynApp is being dropped, saving invocations...");
        self.save_invocations();
    }
}

impl eframe::App for RetsynApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // handling key events here to stay responsive
        self.handle_key_events_and_navigation(ctx);

        if let Some(last_time) = self.last_input_time
            && last_time.elapsed() >= self.debounce_duration
        {
            if self.search_text != self.last_search_text {
                info!("Updating debounced search");
                self.update_search();
                self.last_input_time = None;
            } else {
                // if we remove this repaint request, things feel a lot less responsive
                ctx.request_repaint();
            }
        } else {
            self.retrieve_results();
        }

        // drawing the main UI after updating
        self.draw_main_ui(ctx);
    }
}
