mod cli;
mod config;

use crate::cli::Cli;
use crate::config::Config;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match Config::try_from(cli) {
        Ok(config) => run(config),
        Err(error) => {
            panic!("{error}");
        }
    }
}

fn run(config: Config) {
    dbg!(config);
}
