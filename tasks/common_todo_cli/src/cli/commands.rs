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
    Read { id: String },
    Update { id: String, text: String },
    Delete { id: String },
    List,
    Mark { id: String, is_ready: String },
}
