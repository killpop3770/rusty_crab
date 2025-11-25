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
    Read { id: i32 },
    Update { id: i32, text: String },
    Delete { id: i32 },
    List,
    Mark { id: i32, is_ready: String },
}
