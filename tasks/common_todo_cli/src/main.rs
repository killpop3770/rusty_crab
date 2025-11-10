pub mod cli;

use clap::Parser;
use common_todo_cli::{CommonTodo, db::JsonStorage, gui::gui_mode};

use crate::cli::{Args, cli_mode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, friend!");

    let args = Args::parse();
    let storage = JsonStorage::new()?;
    let todo_app = CommonTodo::new(storage);

    if args.gui_mode {
        gui_mode().await
    } else if let Some(command) = args.command {
        cli_mode(command, &todo_app)
    } else {
        gui_mode().await
    }
}
