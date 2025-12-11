use clap::Parser as _;
use color_eyre::eyre::{self, Result};
use directories::ProjectDirs;
use eframe::egui;
use retsyn::{cli::Cli, config::Conf, fulltext_index::FulltextIndex, ui::retsyn_app::RetsynApp};
use std::process::exit;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn setup_tracing() -> Result<(), Box<dyn std::error::Error>> {
    // Get XDG cache directory for log files
    // TODO replace with PROJECT_DIRS
    let project_dir = ProjectDirs::from("org", "symplasma", "retsyn")
        .ok_or("Could not determine project directory")?;
    let log_dir = project_dir.data_dir();

    // Create cache directory if it doesn't exist
    std::fs::create_dir_all(&log_dir)?;

    // Set up file appender
    let file_appender = tracing_appender::rolling::daily(log_dir, "retsyn.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Set up tracing subscriber with both console and file output
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "retsyn=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false),
        )
        .init();

    // Keep the guard alive for the duration of the program
    std::mem::forget(_guard);

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    // Initialize tracing. We're doing this after reading args from the CLI so we can set the log file path if requested
    #[expect(
        clippy::print_stderr,
        reason = "If we cannot setup tracing, we need to let the user know, but we cannot do so via tracing."
    )]
    if let Err(e) = setup_tracing() {
        eprintln!("Failed to setup tracing: {}", e);
    }

    // Handle --default-config flag
    if cli.default_config {
        #[expect(
            clippy::print_stderr,
            clippy::print_stdout,
            reason = "We want to notify the user on the CLI directly rather than trace these actions."
        )]
        // TODO unify error handling and return error to be displayed later
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
        #[expect(
            clippy::print_stderr,
            clippy::print_stdout,
            reason = "We want to notify the user on the CLI directly rather than trace these actions."
        )]
        // TODO unify error handling and return error to be displayed later
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

    info!("Running native app...");
    eframe::run_native(
        "Search App",
        options,
        Box::new(|cc| Ok(Box::new(RetsynApp::new(cc)))),
    )
    .map_err(|e| eyre::eyre!("{:?}", e))?;

    Ok(())
}
