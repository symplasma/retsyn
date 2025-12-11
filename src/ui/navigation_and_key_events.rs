use crate::ui::retsyn_app::{RetsynApp, UiScreenMode};

impl RetsynApp {
    pub(crate) fn handle_key_events_and_navigation(&mut self, ctx: &egui::Context) {
        // Toggle config screen with Ctrl+,
        if ctx.input(|i| i.key_pressed(egui::Key::Comma) && i.modifiers.ctrl) {
            let mode = if self.show_config() {
                UiScreenMode::Main
            } else {
                UiScreenMode::Config
            };
            self.set_ui_screen_mode(mode);
            return;
        }

        // Toggle help screen with Ctrl+H or Ctrl+?
        if ctx.input(|i| {
            (i.key_pressed(egui::Key::H) && i.modifiers.ctrl)
                || (i.key_pressed(egui::Key::Questionmark) && i.modifiers.ctrl)
        }) {
            let mode = if self.show_help() {
                UiScreenMode::Main
            } else {
                UiScreenMode::Help
            };
            self.set_ui_screen_mode(mode);
            return;
        }

        // Toggle preview pane with Ctrl+P
        if ctx.input(|i| i.key_pressed(egui::Key::P) && i.modifiers.ctrl) {
            self.show_preview = !self.show_preview;
            return;
        }

        // Return to main screen via escape if we're on any other screen
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            if !matches!(self.ui_screen_mode(), UiScreenMode::Main) {
                self.set_ui_screen_mode(UiScreenMode::Main);
                return;
            }
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

        if self.search_text.is_empty() {
            return;
        }

        let item_count = self
            .matched_items
            .as_ref()
            .and_then(|(m, _errors)| Ok(m.len()))
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
}
