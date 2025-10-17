use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create { text: String },
    Read { id: u32 },
    Update { id: u32, text: String },
    Delete { id: u32 },
    List,
    Mark { id: u32, is_ready: String },
}
