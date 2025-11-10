use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(long, short = 'g')]
    pub gui_mode: bool,
}

#[derive(Subcommand)]
pub enum Command {
    Create { text: String },
    Read { id: u32 },
    Update { id: u32, text: String },
    Delete { id: u32 },
    List,
    Mark { id: u32, is_ready: String },
}
