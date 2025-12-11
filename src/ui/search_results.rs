use egui::RichText;
use tracing::info;

use crate::ui::retsyn_app::RetsynApp;

impl RetsynApp {
    pub(crate) fn draw_search_results(
        &mut self,
        clicked_item: &mut Option<(usize, bool)>,
        ui: &mut egui::Ui,
    ) {
        if let Ok((matched_items, _errors)) = &self.matched_items {
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

                        // we need to check double click first or we'll never detect it since it contains a single click
                        if response.double_clicked() {
                            // Double click: activate the item
                            // TODO find out why the modifiers shift value seems to be inverted
                            let shift_held = ui.input(|i| !i.modifiers.shift);
                            *clicked_item = Some((idx, shift_held));
                        } else if response.clicked() {
                            // Single click: just select the item
                            self.selected_index = Some(idx);
                        }
                    });

                    // draw the item snippet
                    if self.show_snippets {
                        item.draw_snippet(ui);
                    }
                });
            }
        }

        // act on the double clicked item
        if let Some((idx, shift_held)) = clicked_item {
            info!("opening item {} with shift_held = {}", idx, shift_held);
            self.open_item(*idx, *shift_held);
        }

        self.scroll_to_selected = false;
    }
}
