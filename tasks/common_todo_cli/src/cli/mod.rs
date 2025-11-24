pub mod commands;

pub use commands::{Args, Command};
use common_todo_cli::{CommonTodo, db::AsyncStorage, ready_or_not};

pub async fn cli_mode<S: AsyncStorage>(
    command: Command,
    todo_app: &CommonTodo<S>,
) -> anyhow::Result<()> {
    println!("cli mode is active");

    match command {
        Command::Create { text } => {
            let task = todo_app.create(text).await?;
            println!("Created task #{} !", task.id);
            Ok(())
        }
        Command::Read { id } => {
            let task = todo_app.read(id).await?;
            println!(
                "Your task #{} is {}: {}",
                task.id,
                ready_or_not(task.is_ready),
                task.value
            );
            Ok(())
        }
        Command::Update { id, text } => {
            let _ = todo_app.update(id, text).await?;
            println!("Task #{id} is updated !");
            Ok(())
        }
        Command::Delete { id } => {
            todo_app.delete(id).await?;
            println!("Task #{id} is deleted !");
            Ok(())
        }
        Command::List => {
            let tasks = todo_app.list().await?;
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
        Command::Mark { id, is_ready } => {
            let is_ready = is_ready.parse::<bool>()?;
            let _ = todo_app.mark_ready_or_not(id, is_ready).await?;
            println!("Task #{id} is marked as {} !", ready_or_not(is_ready));
            Ok(())
        }
    }
}
