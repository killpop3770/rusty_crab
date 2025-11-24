pub mod cli;

use clap::Parser;
use common_todo_cli::{CommonTodo, db::postgresql_storage::PostgresStorage, gui::gui_mode};
use sqlx::postgres::PgPoolOptions;

use crate::cli::{Args, cli_mode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, friend!");

    let args = Args::parse();
    // let storage = JsonStorage::new()?;

    let database_url = "postgresql://admin:admin@localhost:5432/todoapp".to_string();
    let table_name = "tasks".to_string();
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    println!("Hello, friend! 1");
    let storage = PostgresStorage::new(pool, table_name);
    storage.init_table().await.expect("Failed to init table");
    let todo_app = CommonTodo::new(storage);

    println!("Hello, friend! 2");
    if args.gui_mode {
        gui_mode(todo_app).await
    } else if let Some(command) = args.command {
        cli_mode(command, &todo_app).await
    } else {
        gui_mode(todo_app).await
    }
}

// TODO: Создать конфиг файлы
// TODO: Создать фабрики для создания подключений в зависимости от флага компиляции в main файле
