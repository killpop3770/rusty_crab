pub mod cli;

use clap::Parser;
#[cfg(feature = "postgres")]
use common_todo_cli::StorageType;
use common_todo_cli::{CommonTodo, StorageFactory, db::config::AppConfig, gui::gui_mode};

use crate::cli::{Args, cli_mode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, friend!");

    let args = Args::parse();

    let config = AppConfig::new().expect("Can not create app config");

    let storage_type = {
        if cfg!(feature = "json") {
            StorageType::Json
        } else if cfg!(feature = "postgres") {
            StorageType::Postgres
        } else {
            panic!()
        }
    };

    let storage = StorageFactory::create(storage_type, config).await?;

    let todo_app = CommonTodo::new(storage);

    if args.gui_mode {
        gui_mode(todo_app).await
    } else if let Some(command) = args.command {
        cli_mode(command, &todo_app).await
    } else {
        gui_mode(todo_app).await
    }
}

// TODO: доделать json_storage, mongodb (возможность создать скрипт для сборки окружения + компиляции с флагами)
// TODO: вынести фабрику в отдельный файл
// TODO: доделать фабрику и вынести зависимости для отдельных фич !
// TODO: посмотреть можно ли переписать код под generic типы вместо dyn Trait

// TODO: Вынести трейты, use_cases и базовые сущности в отдельный модуль core
// TODO: separate to web_gui(with handlers) + routes
// TODO: отдельная реализация common сервера, вместо axum
// TODO: Для UI сделать новую фабрику?
