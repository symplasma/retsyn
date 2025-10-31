use std::path::PathBuf;

use egui::{Color32, Frame};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use tantivy::{
    DateTime, TantivyDocument,
    schema::{Field, Value},
    snippet::Snippet,
};
use time::{UtcOffset, format_description::well_known::Rfc2822};
use tracing::{info, warn};

use crate::fulltext_index::FulltextIndex;

#[derive(Debug)]
pub(crate) struct SearchResult {
    source: String,
    indexed_at: DateTime,
    path: String,
    title: String,
    body: Field,
    snippet: Snippet,
    tantivy_doc: TantivyDocument,
}

impl SearchResult {
    pub(crate) fn new(
        fulltext_index: &FulltextIndex,
        doc: TantivyDocument,
        snippet: Snippet,
    ) -> Self {
        Self {
            source: doc
                .get_first(fulltext_index.source_field)
                .map(|t| t.as_str())
                .flatten()
                .unwrap_or_default()
                .to_owned(),
            indexed_at: doc
                .get_first(fulltext_index.indexed_at_field)
                .map(|t| t.as_datetime())
                .flatten()
                .unwrap_or_default()
                .to_owned(),
            path: doc
                .get_first(fulltext_index.path_field)
                .map(|t| t.as_str())
                .flatten()
                .unwrap_or_default()
                .to_owned(),
            title: doc
                .get_first(fulltext_index.title_field)
                .map(|t| t.as_str())
                .flatten()
                .unwrap_or_default()
                .to_owned(),
            body: fulltext_index.body_field,
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

    pub(crate) fn body(&self) -> String {
        // TODO might want to grab this from the file directly rather than storing the whole field
        self.tantivy_doc
            .get_first(self.body)
            .map(|t| t.as_str())
            .flatten()
            .unwrap_or_default()
            .to_owned()
    }

    pub(crate) fn draw_snippet(&self, ui: &mut egui::Ui) {
        Frame::NONE
            .fill(Color32::from_rgb(240, 240, 240))
            .inner_margin(4.0)
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    // the below was kinda working, but it needs an update for the new version of egui
                    // let width = ui.fonts(|f| f.glyph_width(&TextStyle::Body.resolve(ui.style()), ' '));
                    // TODO adjust spacing to make it more visually pleasing
                    let width = 1.0;
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
    }

    pub(crate) fn draw_preview_area(&self, ui: &mut egui::Ui) {
        ui.style_mut().url_in_tooltip = true;
        let text = self.body();
        let mut cache = CommonMarkCache::default();
        CommonMarkViewer::new().show(ui, &mut cache, &text);
    }

    pub(crate) fn open(&self) {
        info!("Revealing item: {}", self.path);
        if let Some(path) = PathBuf::from(self.path.clone()).parent() {
            // TODO handle errors in open and display them to the user in the UI
            match open::with(path, "xdg-open") {
                Ok(_) => info!("successfully opened item: {}", self.path),
                Err(e) => warn!("unable to open item: {}", e),
            }
        }
    }

    pub(crate) fn reveal(&self) {
        info!("Opening item: {}", self.path);
        // TODO handle errors in reveal and display them to the user in the UI
        match open::that(self.path.clone()) {
            Ok(_) => info!("successfully revealed item: {}", self.path),
            Err(e) => warn!("unable to reveal item: {}", e),
        }
    }
}
