use egui::{Color32, TextStyle};
use tantivy::{
    TantivyDocument,
    schema::{Field, Value as _},
    snippet::Snippet,
};

#[derive(Debug)]
pub(crate) struct SearchResult {
    title: String,
    snippet: Snippet,
    tantivy_doc: TantivyDocument,
}

impl SearchResult {
    pub(crate) fn new(title: Field, doc: TantivyDocument, snippet: Snippet) -> Self {
        Self {
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

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn draw_snippet(&self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            let width = ui.fonts(|f| f.glyph_width(&TextStyle::Body.resolve(ui.style()), ' '));
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
        });
    }
}
