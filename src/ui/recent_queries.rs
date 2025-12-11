use std::time::Instant;

use crate::ui::retsyn_app::RetsynApp;

impl RetsynApp {
    pub(crate) fn draw_recent_queries(&mut self, ui: &mut egui::Ui) {
        ui.heading("Recent Queries");
        ui.add_space(5.0);

        for (idx, invocation) in self.recent_queries.iter().enumerate() {
            let is_selected = self.selected_index == Some(idx);
            let response = ui.selectable_label(is_selected, invocation.title.clone());

            if response.clicked() {
                self.search_text = invocation.query.clone();
                self.last_input_time = Some(Instant::now());
            }
        }
    }
}
