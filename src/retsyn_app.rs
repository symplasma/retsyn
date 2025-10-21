use std::{
    process::exit,
    sync::LazyLock,
    time::{Duration, Instant},
};

use confique::{
    Config,
    toml::{self, FormatOptions},
};
use directories::ProjectDirs;
use egui::{Align, Button, Color32, Layout, RichText};
use tantivy::TantivyError;

use crate::{config::Conf, fulltext_index::FulltextIndex, search_result::SearchResult};

pub(crate) static PROJECT_DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("org", "symplasma", "retsyn").expect("should be able to create project dir")
});

type SearchResultList = Vec<SearchResult>;

pub struct RetsynApp {
    search_text: String,
    last_search_text: String,
    matched_items: Result<SearchResultList, TantivyError>,
    selected_index: Option<usize>,
    last_input_time: Option<Instant>,
    debounce_duration: Duration,
    recent_queries: Vec<String>,
    scroll_to_selected: bool,
    dark_mode: bool,
    show_snippets: bool,
    show_preview: bool,
    show_help: bool,
    show_config: bool,
    config: Conf,
    config_markdown_files: Vec<String>,
    fulltext_index: Option<FulltextIndex>,
}

impl RetsynApp {
    fn new() -> Result<Self, TantivyError> {
        let config_file = PROJECT_DIRS.config_dir().to_path_buf().join("retsyn.toml");
        let config_exists = Conf::config_exists();

        let config = match Conf::builder().env().file(&config_file).load() {
            Ok(config) => config,
            Err(_) => {
                // If config doesn't exist, create a default one
                Conf::builder().env().load().unwrap_or_else(|_| {
                    // If even that fails, use hardcoded defaults
                    println!("{}", toml::template::<Conf>(FormatOptions::default()));
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

        let fulltext_index = if config_exists {
            let mut index = FulltextIndex::new(config.clone())?;
            index.update()?;
            Some(index)
        } else {
            None
        };

        Ok(Self {
            search_text: String::new(),
            last_search_text: String::new(),
            matched_items: Ok(Vec::new()),
            selected_index: None,
            last_input_time: None,
            debounce_duration: Duration::from_millis(100),
            recent_queries: vec![
                "Recent query 1".to_string(),
                "Recent query 2".to_string(),
                "Recent query 3".to_string(),
            ],
            scroll_to_selected: false,
            dark_mode: false,
            show_snippets: true,
            show_preview: false,
            show_help: false,
            show_config: !config_exists,
            config,
            config_markdown_files,
            fulltext_index,
        })
    }

    /// Returns the currently selected item as a reference
    ///
    /// This is useful to render the preview if it is shown.
    fn selected_item(&self) -> Option<&SearchResult> {
        match self.selected_index {
            Some(selected_index) => match &self.matched_items {
                Ok(items) => items.get(selected_index),
                Err(_) => None,
            },
            None => None,
        }
    }

    fn clear_search(&mut self) {
        self.search_text.clear();
        self.matched_items = Ok(Vec::default());
        self.selected_index = None;
    }

    fn update_search(&mut self) {
        if self.search_text.is_empty() {
            self.matched_items = Ok(Vec::default());
            self.selected_index = None;
        } else if let Some(ref index) = self.fulltext_index {
            self.matched_items = index.search(&self.search_text, 20);
            self.selected_index = if self.matched_items.as_ref().is_ok_and(|m| m.is_empty()) {
                None
            } else {
                Some(0)
            }
        }
        self.last_search_text = self.search_text.clone();
    }

    fn open_item(&mut self, index: usize, reveal: bool) {
        if let Ok(matched_items) = &self.matched_items {
            if index < matched_items.len() {
                let item = &matched_items[index];
                if reveal {
                    item.reveal();
                } else {
                    item.open();
                }

                if !self.search_text.is_empty() && !self.recent_queries.contains(&self.search_text)
                {
                    self.recent_queries.insert(0, self.search_text.clone());
                    if self.recent_queries.len() > 10 {
                        self.recent_queries.truncate(10);
                    }
                }
            }
        }
    }

    fn handle_navigation(&mut self, ctx: &egui::Context) {
        if self.search_text.is_empty() {
            return;
        }

        let item_count = self
            .matched_items
            .as_ref()
            .and_then(|m| Ok(m.len()))
            .unwrap_or_default();
        if item_count == 0 {
            return;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            if let Some(index) = self.selected_index {
                self.selected_index = Some((index + 1).min(item_count - 1));
            } else {
                self.selected_index = Some(0);
            }
            self.scroll_to_selected = true;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            if let Some(index) = self.selected_index {
                if index > 0 {
                    self.selected_index = Some(index - 1);
                }
            } else {
                self.selected_index = Some(0);
            }
            self.scroll_to_selected = true;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Home)) {
            self.selected_index = Some(0);
            self.scroll_to_selected = true;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::End)) {
            self.selected_index = Some(item_count - 1);
            self.scroll_to_selected = true;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            if let Some(index) = self.selected_index {
                // TODO find out why we need to negate shift for correct behavior here
                let shift_held = !(ctx.input(|i| i.modifiers.shift));
                self.open_item(index, shift_held);

                let alt_held = ctx.input(|i| i.modifiers.alt);
                if !alt_held {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
    }

    fn draw_config_screen(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("Retsyn Configuration").size(24.0));
            ui.add_space(20.0);
        });

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.group(|ui| {
                    ui.heading("Markdown Files");
                    ui.add_space(10.0);
                    ui.label("Directories containing loose markdown files to index:");
                    ui.add_space(10.0);

                    let mut to_remove: Option<usize> = None;

                    for (idx, path) in self.config_markdown_files.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}.", idx + 1));

                            let text_edit = egui::TextEdit::singleline(path)
                                .desired_width(ui.available_width() - 120.0);
                            ui.add(text_edit);

                            if ui.button("Browse...").clicked() {
                                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                    *path = folder.to_string_lossy().to_string();
                                }
                            }

                            if ui.button("Remove").clicked() {
                                to_remove = Some(idx);
                            }
                        });
                    }

                    if let Some(idx) = to_remove {
                        self.config_markdown_files.remove(idx);
                    }

                    ui.add_space(10.0);

                    if ui.button("Add Directory").clicked() {
                        self.config_markdown_files.push(String::new());
                    }
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Save Configuration").clicked() {
                        // Convert strings back to PathBuf
                        self.config.markdown_files = self
                            .config_markdown_files
                            .iter()
                            .filter(|s| !s.trim().is_empty())
                            .map(|s| std::path::PathBuf::from(s))
                            .collect();

                        match self.config.save() {
                            Ok(path) => {
                                println!("Configuration saved to: {}", path.display());

                                // Rebuild the index with new configuration
                                match FulltextIndex::new(self.config.clone()) {
                                    Ok(mut index) => {
                                        if let Err(e) = index.update() {
                                            println!("Error updating index: {}", e);
                                        } else {
                                            self.fulltext_index = Some(index);
                                            self.show_config = false;
                                        }
                                    }
                                    Err(e) => {
                                        println!("Error creating index: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error saving configuration: {}", e);
                            }
                        }
                    }

                    if self.fulltext_index.is_some() && ui.button("Cancel").clicked() {
                        // Reload config from file
                        self.config_markdown_files = self
                            .config
                            .markdown_files
                            .iter()
                            .map(|p| p.to_string_lossy().to_string())
                            .collect();
                        self.show_config = false;
                    }
                });

                ui.add_space(20.0);

                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("Configuration will be saved to:")
                            .italics()
                            .size(12.0),
                    );
                    ui.label(
                        RichText::new(Conf::config_path().display().to_string())
                            .monospace()
                            .size(12.0),
                    );
                });
            });
    }

    fn draw_help_screen(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("Retsyn Help").size(24.0));
            ui.add_space(20.0);
        });

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.group(|ui| {
                    ui.heading("Keyboard Shortcuts");
                    ui.add_space(10.0);

                    let shortcuts = vec![
                        ("Ctrl+H or Ctrl+?", "Show/hide this help screen"),
                        ("Ctrl+,", "Show/hide configuration screen"),
                        ("Ctrl+P", "Toggle preview pane"),
                        ("Ctrl+U", "Clear search text"),
                        ("Escape", "Clear search or close window"),
                        ("Ctrl+Q / Ctrl+W / Ctrl+C / Ctrl+D", "Close window"),
                        ("↑ / ↓", "Navigate through search results"),
                        ("Home / End", "Jump to first/last result"),
                        ("Enter", "Open selected item's parent directory"),
                        ("Shift+Enter", "Open selected item directly"),
                        ("Alt+Enter", "Open item and keep window open"),
                    ];

                    for (key, description) in shortcuts {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(key).strong().monospace());
                            ui.label("—");
                            ui.label(description);
                        });
                    }
                });

                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.heading("Search Syntax");
                    ui.add_space(10.0);

                    ui.label("Retsyn uses Tantivy's query parser for full-text search.");
                    ui.add_space(10.0);

                    let syntax_examples = vec![
                        (
                            "simple query",
                            "Search for documents containing these words",
                        ),
                        ("\"exact phrase\"", "Search for an exact phrase"),
                        ("term1 AND term2", "Both terms must be present"),
                        ("term1 OR term2", "Either term must be present"),
                        ("term1 NOT term2", "First term present, second term absent"),
                        ("title:keyword", "Search only in the title field"),
                        ("body:keyword", "Search only in the body field"),
                        ("path:keyword", "Search only in the file path"),
                        ("term*", "Wildcard search (prefix matching)"),
                        ("term~", "Fuzzy search (finds similar terms)"),
                    ];

                    for (syntax, description) in syntax_examples {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(syntax).code());
                            ui.label("—");
                            ui.label(description);
                        });
                    }

                    ui.add_space(10.0);
                    ui.label(RichText::new("Examples:").strong());
                    ui.add_space(5.0);

                    let examples = vec![
                        "rust programming",
                        "\"design patterns\"",
                        "title:architecture AND body:microservices",
                        "path:*/2024/* meeting",
                    ];

                    for example in examples {
                        ui.horizontal(|ui| {
                            ui.label("•");
                            ui.label(RichText::new(example).code());
                        });
                    }
                });

                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.heading("UI Controls");
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Snippets button").strong());
                        ui.label("—");
                        ui.label("Toggle display of search result snippets");
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Preview button").strong());
                        ui.label("—");
                        ui.label("Toggle preview pane (coming soon)");
                    });
                });

                ui.add_space(20.0);

                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("Press Ctrl+H or Escape to close this help screen").italics(),
                    );
                });
            });
    }

    fn draw_main_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_config {
                self.draw_config_screen(ui);
                return;
            }

            if self.show_help {
                self.draw_help_screen(ui);
                return;
            }

            ui.vertical(|ui| {
                ui.add_space(10.0);

                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.search_text)
                        .desired_width(f32::INFINITY)
                        .hint_text("Search..."),
                );

                if response.changed() {
                    self.last_input_time = Some(Instant::now());
                }

                response.request_focus();

                let button_bar = vec![
                    ("Snippets", self.show_snippets),
                    ("Preview", self.show_preview),
                ];

                // add mode toggles
                ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.columns(2, |columns| {
                        for (column, (label, state)) in columns.iter_mut().zip(button_bar) {
                            let button_response = column.add_sized(
                                [column.available_width(), 0.0],
                                Button::new(label).selected(state),
                            );

                            if button_response.clicked() {
                                if label == "Snippets" {
                                    self.show_snippets = !self.show_snippets;
                                } else if label == "Preview" {
                                    self.show_preview = !self.show_preview;
                                }
                            }
                        }
                    });
                });

                ui.add_space(10.0);
                if let Err(query_error) = &self.matched_items {
                    ui.colored_label(Color32::RED, query_error.to_string());
                }

                let mut clicked_item: Option<(usize, bool)> = None;

                let num_columns = if self.show_preview { 2 } else { 1 };

                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.columns(num_columns, |columns| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .id_salt("search_results")
                            .show(&mut columns[0], |ui| {
                                if self.search_text.is_empty() {
                                    self.draw_recent_queries(ui);
                                } else {
                                    self.draw_search_results(&mut clicked_item, ui);
                                }
                            });

                        if self.show_preview {
                            egui::ScrollArea::vertical()
                                .id_salt("preview")
                                .auto_shrink([false, false])
                                .show(&mut columns[1], |ui| match self.selected_item() {
                                    Some(selected_item) => selected_item.draw_preview_area(ui),
                                    None => {
                                        ui.heading("preview");
                                    }
                                });
                        }
                    })
                });
            });
        });
    }

    fn draw_search_results(&mut self, clicked_item: &mut Option<(usize, bool)>, ui: &mut egui::Ui) {
        if let Ok(matched_items) = &self.matched_items {
            for (idx, item) in matched_items.iter().enumerate() {
                ui.vertical(|ui| {
                    // draw the item header
                    ui.horizontal_wrapped(|ui| {
                        let is_selected = self.selected_index == Some(idx);
                        let response =
                            ui.selectable_label(is_selected, RichText::new(item.title()).heading());
                        ui.label(item.path());
                        ui.label(item.indexed_at());

                        if self.scroll_to_selected && is_selected {
                            response.scroll_to_me(Some(egui::Align::Center));
                        }

                        if response.clicked() {
                            let shift_held = ui.input(|i| i.modifiers.shift);
                            *clicked_item = Some((idx, shift_held));
                        }
                    });

                    // draw the item snippet
                    if self.show_snippets {
                        item.draw_snippet(ui);
                    }
                });
            }
        }

        self.scroll_to_selected = false;
    }

    fn draw_recent_queries(&mut self, ui: &mut egui::Ui) {
        ui.heading("Recent Queries");
        ui.add_space(5.0);

        for (idx, query) in self.recent_queries.iter().enumerate() {
            let is_selected = self.selected_index == Some(idx);
            let response = ui.selectable_label(is_selected, query);

            if response.clicked() {
                self.search_text = query.clone();
                self.last_input_time = Some(Instant::now());
            }
        }
    }

    fn handle_key_events(&mut self, ctx: &egui::Context) {
        // Toggle config screen with Ctrl+,
        if ctx.input(|i| i.key_pressed(egui::Key::Comma) && i.modifiers.ctrl) {
            self.show_config = !self.show_config;
            self.show_help = false;
            return;
        }

        // Toggle help screen with Ctrl+H or Ctrl+?
        if ctx.input(|i| {
            (i.key_pressed(egui::Key::H) && i.modifiers.ctrl)
                || (i.key_pressed(egui::Key::Questionmark) && i.modifiers.ctrl)
        }) {
            self.show_help = !self.show_help;
            self.show_config = false;
            return;
        }

        // Toggle preview pane with Ctrl+P
        if ctx.input(|i| i.key_pressed(egui::Key::P) && i.modifiers.ctrl) {
            self.show_preview = !self.show_preview;
            return;
        }

        // If config screen is showing, Escape closes it (only if index exists)
        if self.show_config {
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) && self.fulltext_index.is_some() {
                self.show_config = false;
            }
            return;
        }

        // If help screen is showing, Escape closes it
        if self.show_help {
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.show_help = false;
            }
            return;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::U) && i.modifiers.ctrl)
            && !self.search_text.is_empty()
        {
            self.clear_search();
            return;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            if self.search_text.is_empty() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            } else {
                self.clear_search();
            }
            return;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Q) && i.modifiers.ctrl)
            || ctx.input(|i| i.key_pressed(egui::Key::W) && i.modifiers.ctrl)
            || ctx.input(|i| i.key_pressed(egui::Key::C) && i.modifiers.ctrl)
            || ctx.input(|i| i.key_pressed(egui::Key::D) && i.modifiers.ctrl)
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl Default for RetsynApp {
    fn default() -> Self {
        Self::new().expect("should be able to make a new RetsynApp")
    }
}

impl eframe::App for RetsynApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set theme based on dark_mode toggle
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        self.handle_key_events(ctx);

        self.handle_navigation(ctx);

        if let Some(last_time) = self.last_input_time {
            if last_time.elapsed() >= self.debounce_duration
                && self.search_text != self.last_search_text
            {
                self.update_search();
                self.last_input_time = None;
            } else {
                ctx.request_repaint();
            }
        }

        self.draw_main_ui(ctx);
    }
}
