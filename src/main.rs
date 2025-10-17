use clap::Parser;
use eframe::egui;
use retsyn::config::Conf;
use retsyn::fulltext_index::FulltextIndex;
use retsyn::retsyn_app::RetsynApp;
use std::process::exit;

#[derive(Parser)]
#[command(name = "retsyn")]
#[command(about = "A full text search program", long_about = None)]
struct Cli {
    /// Create a default config template and write it to the default config path
    #[arg(long)]
    default_config: bool,

    /// Clear the search index so that it will be regenerated on the next launch
    #[arg(long)]
    clear_index: bool,
}

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
