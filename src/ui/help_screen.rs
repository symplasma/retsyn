use egui::RichText;

use crate::ui::retsyn_app::RetsynApp;

impl RetsynApp {
    pub(crate) fn draw_help_screen(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("Retsyn Help").size(24.0));
            ui.add_space(20.0);
        });

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.group(|ui| {
                    ui.heading("UI Controls");
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Lenient button").strong());
                        ui.label("—");
                        ui.label("Toggle lenient search query parsing");
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("All/Any button").strong());
                        ui.label("—");
                        ui.label("Toggle require all or any query parameters");
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Exact/Fuzzy button").strong());
                        ui.label("—");
                        ui.label("Choose between Exact, Fuzzy, or Very Fuzzy matching on the title and body fields");
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Snippets button").strong());
                        ui.label("—");
                        ui.label("Toggle display of search result snippets");
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Preview button").strong());
                        ui.label("—");
                        ui.label("Toggle preview pane");
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Help button").strong());
                        ui.label("—");
                        ui.label("Show this screen");
                    });
                });

                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.heading("Keyboard Shortcuts");
                    ui.add_space(10.0);

                    let shortcuts = vec![
                        ("Ctrl+H or Ctrl+?", "Show/hide this help screen"),
                        ("Ctrl+,", "Show/hide configuration screen"),
                        ("Ctrl+P", "Toggle preview pane"),
                        ("Ctrl+U", "Clear search text"),
                        ("Escape", "Clear search or close window"),
                        ("Ctrl+Q / Ctrl+W / Ctrl+C / Ctrl+D", "Close window"),
                        ("↑ / ↓", "Navigate through search results"),
                        ("Home / End", "Jump to first/last result"),
                        ("Enter", "Open selected item directly"),
                        ("Shift+Enter", "Open selected item's parent directory"),
                        ("Alt+Enter", "Open item and keep window open"),
                    ];

                    for (key, description) in shortcuts {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(key).strong().monospace());
                            ui.label("—");
                            ui.label(description);
                        });
                    }
                });

                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.heading("Search Syntax");
                    ui.add_space(10.0);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Retsyn uses");
                        ui.hyperlink_to(
                            "Tantivy's query parser",
                            "https://docs.rs/tantivy/latest/tantivy/query/struct.QueryParser.html",
                        );
                        ui.label("for full-text search.");
                    });
                    ui.add_space(10.0);

                    let syntax_examples = vec![
                        (
                            "simple query",
                            "Search for documents containing these words in the title or body",
                        ),
                        ("\"exact phrase\"", "Search for the exact phrase"),
                        (
                            "\"exact phrase\"~2",
                            "Search for the exact phrase with up to two words between",
                        ),
                        ("term1 AND term2", "Both terms must be present"),
                        ("term1 OR term2", "Either term must be present"),
                        ("+term1 -term2", "First term present, second term absent"),
                        ("title:keyword", "Search only in the title field"),
                        ("body:keyword", "Search only in the body field"),
                        ("path:keyword", "Search only in the file path"),
                        ("title: IN [a b c]", "Search for title is either a, b, or c"),
                        ("\"term\"*", "Wildcard search (prefix matching)"),
                        ("term^2.0", "Boost these terms during ranking"),
                    ];

                    for (syntax, description) in syntax_examples {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(syntax).code());
                            ui.label("—");
                            ui.label(description);
                        });
                    }

                    ui.add_space(10.0);
                    ui.label(RichText::new("Examples:").strong());
                    ui.add_space(5.0);

                    let examples = vec![
                        "rust programming",
                        "\"design patterns\"",
                        "title:architecture AND body:microservices",
                        "path:*/2024/* meeting",
                    ];

                    for example in examples {
                        ui.horizontal(|ui| {
                            ui.label("•");
                            ui.label(RichText::new(example).code());
                        });
                    }
                });

                ui.add_space(20.0);

                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("Press Ctrl+H or Escape to close this help screen").italics(),
                    );
                });
            });
    }
}
