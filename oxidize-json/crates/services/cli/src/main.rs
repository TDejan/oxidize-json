mod commands;

use clap::Parser;

use crate::commands::cli::{Cli, Commands};
use crate::commands::read::prettify_json;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Read { file } => {
            // Call the prettify_json function from the read module
            prettify_json(&file);
        }
    }
}
