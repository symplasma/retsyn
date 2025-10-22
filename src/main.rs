use clap::Parser as _;
use eframe::egui;
use retsyn::cli::Cli;
use retsyn::config::Conf;
use retsyn::fulltext_index::FulltextIndex;
use retsyn::retsyn_app::RetsynApp;
use std::process::exit;

fn main() -> eframe::Result {
    let cli = Cli::parse();

    // Handle --default-config flag
    if cli.default_config {
        match Conf::write_default_config() {
            Ok(path) => {
                println!("Default config written to: {}", path.display());
                exit(0);
            }
            Err(e) => {
                eprintln!("Error writing default config: {}", e);
                exit(1);
            }
        }
    }

    // Handle --clear-index flag
    if cli.clear_index {
        match FulltextIndex::clear_index() {
            Ok(()) => {
                println!("Search index cleared successfully");
                exit(0);
            }
            Err(e) => {
                eprintln!("Error clearing index: {}", e);
                exit(1);
            }
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Search App"),
        ..Default::default()
    };

    eframe::run_native(
        "Search App",
        options,
        Box::new(|_cc| Ok(Box::new(RetsynApp::default()))),
    )
}
