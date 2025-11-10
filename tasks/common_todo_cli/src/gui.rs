use anyhow::Ok;

use axum::{
    Json, Router,
    extract::{Path, State},
    response::Html,
    routing::get,
};
use std::sync::Arc;

use crate::{CommonTodo, db::Storage, ready_or_not};

pub async fn gui_mode<S>(todo_app: CommonTodo<S>) -> anyhow::Result<()>
where
    S: Storage,
{
    println!("gui mode is active");

    let shared_state = Arc::new(todo_app.storage);

    let app = Router::new()
        .route("/", get(handler))
        .route("/read", get(test_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn test_handler(State(todo_app): State<Arc<dyn Storage>>) -> Result<String, anyhow::Error> {
    Ok("value".to_string())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn get_one_task<S>(
    State(todo_app): State<Arc<CommonTodo<S>>>,
    Path(id): Path<u32>,
) -> Result<Html<String>, anyhow::Error>
where
    S: Storage,
{
    let task = todo_app.read(id)?;
    Ok(Html(format!(
        "<h2>Your task #{} is {}: {}</h2>",
        task.id,
        ready_or_not(task.is_ready),
        task.value
    )))
}
