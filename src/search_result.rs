use std::path::PathBuf;

use egui::{Color32, Frame, TextStyle};
use tantivy::{
    DateTime, TantivyDocument,
    schema::{Field, Value},
    snippet::Snippet,
};
use time::{UtcOffset, format_description::well_known::Rfc2822};

#[derive(Debug)]
pub(crate) struct SearchResult {
    source: String,
    indexed_at: DateTime,
    path: String,
    title: String,
    snippet: Snippet,
    tantivy_doc: TantivyDocument,
}

impl SearchResult {
    pub(crate) fn new(
        source: Field,
        indexed_at: Field,
        path: Field,
        title: Field,
        doc: TantivyDocument,
        snippet: Snippet,
    ) -> Self {
        Self {
            source: doc
                .get_first(source)
                .map(|t| t.as_str())
                .flatten()
                .unwrap_or_default()
                .to_owned(),
            indexed_at: doc
                .get_first(indexed_at)
                .map(|t| t.as_datetime())
                .flatten()
                .unwrap_or_default()
                .to_owned(),
            path: doc
                .get_first(path)
                .map(|t| t.as_str())
                .flatten()
                .unwrap_or_default()
                .to_owned(),
            title: doc
                .get_first(title)
                .map(|t| t.as_str())
                .flatten()
                .unwrap_or_default()
                .to_owned(),
            snippet,
            tantivy_doc: doc,
        }
    }

    pub(crate) fn indexed_at(&self) -> String {
        self.indexed_at
            .into_offset(
                UtcOffset::current_local_offset()
                    .expect("should be able to get the user's timezone offset"),
            )
            .format(&Rfc2822)
            .expect("should be able to format indexed at timestamp into RFC3339")
    }

    pub(crate) fn path(&self) -> &str {
        &self.path
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn draw_snippet(&self, ui: &mut egui::Ui) {
        Frame::NONE
            .fill(Color32::from_rgb(240, 240, 240))
            .inner_margin(4.0)
            .show(ui, |ui| {
                let available_width = ui.available_width();
                ui.add_sized([available_width, 0.0], |ui: &mut egui::Ui| {
                    ui.horizontal_wrapped(|ui| {
                        // TODO adjust spacing to make it more visually pleasing
                        let width =
                            ui.fonts(|f| f.glyph_width(&TextStyle::Body.resolve(ui.style()), ' '));
                        ui.spacing_mut().item_spacing.x = width;

                        let mut start_from = 0;
                        for fragment_range in self.snippet.highlighted() {
                            ui.label(&self.snippet.fragment()[start_from..fragment_range.start]);
                            ui.colored_label(
                                Color32::BLUE,
                                &self.snippet.fragment()[fragment_range.clone()],
                            );
                            start_from = fragment_range.end;
                        }
                    })
                    .response
                });
            });
    }

    pub(crate) fn open(&self) {
        println!("Revealing item: {}", self.path);
        if let Some(path) = PathBuf::from(self.path.clone()).parent() {
            // TODO handle errors in open and display them to the user
            open::with(path, "xdg-open");
        }
    }

    pub(crate) fn reveal(&self) {
        println!("Opening item: {}", self.path);
        // TODO handle errors in open and display them to the user
        open::that(self.path.clone());
    }
}
