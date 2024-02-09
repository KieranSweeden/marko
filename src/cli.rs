use clap::Parser;

/// marko, the markdown server/watcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the target file
    #[arg(short, long)]
    pub path: String,
}
