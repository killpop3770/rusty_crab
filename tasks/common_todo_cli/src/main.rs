pub mod cli;

use clap::Parser;
use common_todo_cli::{
    CommonTodo,
    config::app_config::AppConfig,
    db::{StorageImpl, create_storage},
    gui::gui_mode,
};

use crate::cli::{Args, cli_mode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, friend!");

    let args = Args::parse();

    let config = AppConfig::new().expect("Can not create app config");

    let storage: StorageImpl = create_storage(config).await?;

    let todo_app = CommonTodo::new(storage);

    if args.gui_mode {
        gui_mode(todo_app).await
    } else if let Some(command) = args.command {
        cli_mode(command, &todo_app).await
    } else {
        gui_mode(todo_app).await
    }
}

//
// cargo run -- list -> json
// cargo run --features "mongodb" --no-default-features -> mongodb
//
// TODO: Вынести трейты, use_cases и базовые сущности в отдельный модуль core
// TODO: separate to web_gui(with handlers) + routes
// TODO: отдельная реализация common сервера, вместо axum
// TODO: Для UI сделать новую фабрику?
