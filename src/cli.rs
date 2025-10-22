use clap::Parser;

#[derive(Parser)]
#[command(name = "retsyn")]
#[command(about = "A full text search program", long_about = None)]
pub struct Cli {
    /// Create a default config template and write it to the default config path
    #[arg(long)]
    pub default_config: bool,

    /// Clear the search index so that it will be regenerated on the next launch
    #[arg(long)]
    pub clear_index: bool,
}
