use clap::{Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(name = "oxidize-json")]
#[command(about = "A CLI tool to work with JSON and JSON Schema", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands, // Use #[command(subcommand)] for subcommands
}

#[derive(Subcommand, Clone)] // Derive Clone and Subcommand for Commands
pub enum Commands {
    Read { file: String },
}
