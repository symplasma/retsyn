use std::process::exit;

use egui::RichText;
use tracing::warn;

use crate::{config::Conf, ui::retsyn_app::RetsynApp};

impl RetsynApp {
    pub(crate) fn draw_config_screen(&mut self, ui: &mut egui::Ui) {
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
                                .desired_width(ui.available_width() - 140.0);
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
                                #[expect(
                                    clippy::print_stdout,
                                    reason = "We need to notify the user that the file was saved."
                                )]
                                // we're scoping the expect above to only this print statement
                                {
                                    println!("Configuration saved to: {}", path.display());
                                }

                                // TODO restart the index with new configuration. We need to add the ability to gracefully shutdown worker threads so that we can restart.
                                // for now, we'll just exit
                                exit(0)
                            }
                            Err(e) => {
                                warn!("Error saving configuration: {}", e);
                            }
                        }
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
}
