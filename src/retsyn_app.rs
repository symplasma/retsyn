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
use egui::Color32;
use tantivy::TantivyError;

use crate::{config::Conf, fulltext_index::FulltextIndex, search_result::SearchResult};

pub(crate) static PROJECT_DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("org", "symplasma", "retsyn").expect("should be able to create project dir")
});

type SearchResultList = Vec<SearchResult>;

pub struct RetsynApp {
    config: Conf,
    search_text: String,
    last_search_text: String,
    matched_items: Result<SearchResultList, TantivyError>,
    selected_index: Option<usize>,
    last_input_time: Option<Instant>,
    debounce_duration: Duration,
    recent_queries: Vec<String>,
    scroll_to_selected: bool,
    dark_mode: bool,
    fulltext_index: FulltextIndex,
}

impl RetsynApp {
    fn new() -> Result<Self, TantivyError> {
        let config_file = PROJECT_DIRS.config_dir().to_path_buf().join("retsyn.toml");

        let config = match Conf::builder().env().file(config_file).load() {
            Ok(config) => config,
            Err(_) => {
                // TODO make this something that can be invoked via a CLI option
                // create the config template
                println!("{}", toml::template::<Conf>(FormatOptions::default()));
                exit(0)
            }
        };

        let fulltext_index = FulltextIndex::new(&config)?;
        // TODO need to move this into a separate thread
        fulltext_index.update()?;

        Ok(Self {
            config,
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
            fulltext_index,
        })
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
        } else {
            self.matched_items = self.fulltext_index.search(&self.search_text, 20);
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
            }
        }
    }

    fn draw_main_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
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

                ui.add_space(10.0);
                if let Err(query_error) = &self.matched_items {
                    ui.colored_label(Color32::RED, query_error.to_string());
                }

                let mut clicked_item: Option<(usize, bool)> = None;

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.search_text.is_empty() {
                            self.draw_recent_queries(ui);
                        } else {
                            self.draw_search_results(&mut clicked_item, ui);
                        }
                    });
            });
        });
    }

    fn draw_search_results(&mut self, clicked_item: &mut Option<(usize, bool)>, ui: &mut egui::Ui) {
        if let Ok(matched_items) = &self.matched_items {
            for (idx, item) in matched_items.iter().enumerate() {
                ui.vertical(|ui| {
                    // draw the item header
                    ui.horizontal(|ui| {
                        let is_selected = self.selected_index == Some(idx);
                        let response = ui.selectable_label(is_selected, item.title());
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
                    item.draw_snippet(ui);
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
