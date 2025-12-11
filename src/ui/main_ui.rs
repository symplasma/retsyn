use egui::{Align, Button, Color32, DragValue, Layout, OutputCommand, ProgressBar};
use std::time::Instant;
use tracing::debug;

use crate::{
    invocations::{invocation::Action, invocation_list::InvocationList},
    model::fulltext_index::IndexStatus,
    ui::retsyn_app::{RetsynApp, UiScreenMode},
};

impl RetsynApp {
    pub(crate) fn draw_main_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_config() {
                self.draw_config_screen(ui);
                return;
            }

            if self.show_help() {
                self.draw_help_screen(ui);
                return;
            }

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.search_text)
                            .desired_width(ui.available_width() - 56.0)
                            .hint_text("Search..."),
                    );

                    if response.changed() {
                        self.last_input_time = Some(Instant::now());
                    }

                    response.request_focus();
                    if ui.add(DragValue::new(&mut self.limit_results)).changed() {
                        self.update_search();
                    };
                });

                ui.add_space(5.0);

                // add mode toggles
                ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                    // TODO replace with `columns_const`
                    ui.columns_const(
                        |[
                            lenient_col,
                            conjunction_col,
                            fuzz_col,
                            snippet_col,
                            preview_col,
                            help_col,
                        ]| {
                            if lenient_col
                                .add_sized(
                                    [lenient_col.available_width(), 0.0],
                                    Button::new("Lenient").selected(self.lenient),
                                )
                                .clicked()
                            {
                                self.lenient = !self.lenient;
                                self.update_search();
                            };

                            if conjunction_col
                                .add_sized([conjunction_col.available_width(), 0.0], {
                                    let button_name =
                                        if self.query_conjunction { "All" } else { "Any" };
                                    Button::new(button_name).selected(self.query_conjunction)
                                })
                                .clicked()
                            {
                                self.query_conjunction = !self.query_conjunction;
                                self.update_search();
                            };

                            if fuzz_col
                                .add_sized([preview_col.available_width(), 0.0], {
                                    let (name, selected) = if self.fuzziness == 1 {
                                        ("Fuzzy", true)
                                    } else if self.fuzziness == 2 {
                                        ("Very Fuzzy", true)
                                    } else {
                                        self.fuzziness = 0;
                                        ("Exact", false)
                                    };
                                    Button::new(name).selected(selected)
                                })
                                .clicked()
                            {
                                // Levenshtein values from 0 to 2 inclusive are supported
                                self.fuzziness = (self.fuzziness + 1) % 3;
                                self.update_search();
                            };

                            if snippet_col
                                .add_sized(
                                    [snippet_col.available_width(), 0.0],
                                    Button::new("Snippets").selected(self.show_snippets),
                                )
                                .clicked()
                            {
                                self.show_snippets = !self.show_snippets;
                            };

                            if preview_col
                                .add_sized(
                                    [preview_col.available_width(), 0.0],
                                    Button::new("Preview").selected(self.show_preview),
                                )
                                .clicked()
                            {
                                self.show_preview = !self.show_preview;
                            };

                            if help_col
                                .add_sized(
                                    [help_col.available_width(), 0.0],
                                    Button::new("Help").selected(false),
                                )
                                .clicked()
                            {
                                self.set_ui_screen_mode(UiScreenMode::Help);
                            };
                        },
                    );
                });

                // draw index status
                ui.add_space(10.0);

                match &self.index_status {
                    IndexStatus::Initializing
                    | IndexStatus::CollectingPaths
                    | IndexStatus::FilteringPaths => {
                        ui.label("Preparing to index...");
                    }
                    IndexStatus::UpdatingIndex {
                        indexed,
                        total,
                        committing_updates,
                        file_path,
                    } => {
                        let progress = (*indexed as f32) / (*total as f32);
                        ui.horizontal(|ui| {
                            ui.add(
                                ProgressBar::new(progress)
                                    .desired_width(ui.available_width() - 100.0),
                            );
                            ui.add_space(5.0);
                            ui.label(format!("{}/{}", indexed, total));
                        });
                        ui.add_space(5.0);
                        if *committing_updates {
                            ui.colored_label(Color32::BLUE, "Committing updates...");
                        } else {
                            ui.label(file_path);
                        }
                    }
                    IndexStatus::UpToDate => {
                        ui.label("Done indexing");
                    }
                };

                // draw query errors
                ui.add_space(10.0);
                match &self.matched_items {
                    // show query parsing errors in lenient mode
                    Ok((_results, query_errors)) => {
                        let mut indent = String::default();
                        if query_errors.len() > 0 && self.lenient {
                            indent = "  ".to_owned();
                            ui.colored_label(
                                Color32::RED,
                                format!("{} query errors", query_errors.len()),
                            );
                        }
                        for query_error in query_errors {
                            ui.colored_label(
                                Color32::RED,
                                format!("{}{}", indent, query_error.to_string()),
                            );
                        }
                    }

                    // some other error during search
                    Err(query_error) => {
                        ui.colored_label(Color32::RED, query_error.to_string());
                    }
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
                            let mut invocations: InvocationList = Default::default();

                            egui::ScrollArea::vertical()
                                .id_salt("preview")
                                .auto_shrink([false, false])
                                .show(&mut columns[1], |ui| match self.selected_item() {
                                    Some(selected_item) => {
                                        selected_item.draw_preview_area(ui);
                                        ui.ctx().output(|o| {
                                            for command in &o.commands {
                                                match command {
                                                    OutputCommand::OpenUrl(open_url) => {
                                                        invocations.add_invocation(
                                                            Action::OpenLink,
                                                            &self.search_text,
                                                            &selected_item.path,
                                                            &selected_item.title,
                                                            &open_url.url,
                                                        );
                                                        debug!(
                                                            "clicked url: {} in {} {}",
                                                            open_url.url,
                                                            selected_item.title,
                                                            selected_item.path
                                                        )
                                                    }
                                                    // right now we only care about link clicks
                                                    _ => (),
                                                    // OutputCommand::CopyText(_) => todo!(),
                                                    // OutputCommand::CopyImage(color_image) => todo!(),
                                                }
                                            }
                                        });
                                    }
                                    None => {
                                        ui.heading("preview");
                                    }
                                });

                            // add invocations to our invocation list
                            self.invocations.append(&mut invocations);
                        }
                    })
                });
            });
        });
    }
}
