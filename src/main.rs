use eframe::egui;
use std::time::{Duration, Instant};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Search App"),
        ..Default::default()
    };

    eframe::run_native(
        "Search App",
        options,
        Box::new(|_cc| Ok(Box::new(SearchApp::default()))),
    )
}

struct SearchApp {
    search_text: String,
    last_search_text: String,
    matched_items: Vec<String>,
    selected_index: Option<usize>,
    last_input_time: Option<Instant>,
    debounce_duration: Duration,
    recent_queries: Vec<String>,
    scroll_to_selected: bool,
}

impl SearchApp {
    fn new() -> Self {
        Self {
            search_text: String::new(),
            last_search_text: String::new(),
            matched_items: Vec::new(),
            selected_index: None,
            last_input_time: None,
            debounce_duration: Duration::from_millis(100),
            recent_queries: vec![
                "Recent query 1".to_string(),
                "Recent query 2".to_string(),
                "Recent query 3".to_string(),
            ],
            scroll_to_selected: false,
        }
    }

    fn update_search(&mut self) {
        if self.search_text.is_empty() {
            self.matched_items.clear();
            self.selected_index = None;
        } else {
            self.matched_items = self.perform_search(&self.search_text);
            if !self.matched_items.is_empty() {
                self.selected_index = Some(0);
            } else {
                self.selected_index = None;
            }
        }
        self.last_search_text = self.search_text.clone();
    }

    fn perform_search(&self, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        for i in 1..=20 {
            results.push(format!("Item {} matching '{}'", i, query));
        }
        results
    }

    fn open_item(&mut self, index: usize, reveal: bool) {
        if index < self.matched_items.len() {
            let item = &self.matched_items[index];
            if reveal {
                println!("Revealing item: {}", item);
            } else {
                println!("Opening item: {}", item);
            }

            if !self.search_text.is_empty() && !self.recent_queries.contains(&self.search_text) {
                self.recent_queries.insert(0, self.search_text.clone());
                if self.recent_queries.len() > 10 {
                    self.recent_queries.truncate(10);
                }
            }
        }
    }

    fn handle_navigation(&mut self, ctx: &egui::Context) {
        if self.search_text.is_empty() {
            return;
        }

        let item_count = self.matched_items.len();
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
                let shift_held = ctx.input(|i| i.modifiers.shift);
                self.open_item(index, shift_held);
            }
        }
    }
}

impl Default for SearchApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for SearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Q) && i.modifiers.ctrl)
            || ctx.input(|i| i.key_pressed(egui::Key::W) && i.modifiers.ctrl)
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

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

                let mut clicked_item: Option<(usize, bool)> = None;

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.search_text.is_empty() {
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
                        } else {
                            for (idx, item) in self.matched_items.iter().enumerate() {
                                let is_selected = self.selected_index == Some(idx);
                                let response = ui.selectable_label(is_selected, item);

                                if self.scroll_to_selected && is_selected {
                                    response.scroll_to_me(Some(egui::Align::Center));
                                }

                                if response.clicked() {
                                    let shift_held = ui.input(|i| i.modifiers.shift);
                                    clicked_item = Some((idx, shift_held));
                                }
                            }

                            self.scroll_to_selected = false;
                        }
                    });

                if let Some((idx, shift_held)) = clicked_item {
                    self.open_item(idx, shift_held);
                }
            });
        });
    }
}
