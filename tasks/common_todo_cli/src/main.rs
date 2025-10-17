pub mod cli;

use anyhow::Ok;
use clap::Parser;
use common_todo_cli::{CommonTodo, db::JsonStorage, ready_or_not};

use crate::cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    println!("Hello, friend!");

    let storage = JsonStorage::new()?;
    let todo_app = CommonTodo::new(storage);
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { text } => {
            let task = todo_app.create(text)?;
            println!("Created task #{} !", task.id);
            Ok(())
        }
        Commands::Read { id } => {
            let task = todo_app.read(id)?;
            println!(
                "Your task #{} is {}: {}",
                task.id,
                ready_or_not(task.is_ready),
                task.value
            );
            Ok(())
        }
        Commands::Update { id, text } => {
            let _ = todo_app.update(id, text)?;
            println!("Task #{id} is updated !");
            Ok(())
        }
        Commands::Delete { id } => {
            todo_app.delete(id)?;
            println!("Task #{id} is deleted !");
            Ok(())
        }
        Commands::List => {
            let tasks = todo_app.list()?;
            for task in tasks {
                println!(
                    "Task #{} is {}: {}",
                    task.id,
                    ready_or_not(task.is_ready),
                    task.value
                );
            }
            Ok(())
        }
        Commands::Mark { id, is_ready } => {
            let is_ready = is_ready.parse::<bool>()?;
            let _ = todo_app.mark_ready_or_not(id, is_ready)?;
            println!("Task #{id} is marked as {} !", ready_or_not(is_ready));
            Ok(())
        }
    }
}
