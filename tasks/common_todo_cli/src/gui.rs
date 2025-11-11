use anyhow::Ok;

use axum::{
    Json, Router,
    extract::{Path, State},
    response::Html,
    routing::{delete, get, post, put},
};
use std::sync::Arc;

use crate::{CommonTodo, db::Storage, ready_or_not};

type TodoApp = Arc<dyn Storage + Send + Sync + 'static>;

pub async fn gui_mode<S>(todo_app: CommonTodo<S>) -> anyhow::Result<()>
where
    S: Storage + Send + Sync + 'static,
{
    println!("gui mode is active");

    let shared_state: Arc<dyn Storage + Send + Sync + 'static> = Arc::new(todo_app.storage);

    let app = Router::new()
        .route("/", get(handler))
        .route("/tasks", post(create_task_handler))
        .route("/tasks", get(read_all_tasks_handler))
        .route("/tasks/{id}", get(read_task_by_id_handler))
        .route("/tasks/{id}", put(update_task_handler))
        .route("/tasks/{id}", delete(delete_task_handler))
        .route("/tasks/mark/{id}", post(mark_task_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn create_task_handler(
    State(todo_app): State<TodoApp>,
    Json(payload): Json<String>,
) -> Html<String> {
    let task = todo_app.create(payload).unwrap();
    Html(format!("<h2Created task #{} !</h2>", task.id))
}

async fn read_all_tasks_handler(State(todo_app): State<TodoApp>) -> Html<String> {
    let tasks = todo_app.list().unwrap();
    let mut acc = String::new();
    for task in tasks {
        let temp = format!(
            "<h2>Your task #{} is {}: {}</h2>\n",
            task.id,
            ready_or_not(task.is_ready),
            task.value
        );
        acc.push_str(&temp);
    }
    Html(acc)
}

async fn read_task_by_id_handler(
    State(todo_app): State<TodoApp>,
    Path(id): Path<u32>,
) -> Html<String> {
    let task = todo_app.read(id).unwrap();
    Html(format!(
        "<h2>Your task #{} is {}: {}</h2>",
        task.id,
        ready_or_not(task.is_ready),
        task.value
    ))
}

async fn update_task_handler(
    State(todo_app): State<TodoApp>,
    Path(id): Path<u32>,
    Json(payload): Json<String>,
) -> Html<String> {
    todo_app.update(id, payload).unwrap();
    Html(format!("<h2>Task #{id} is updated !</h2>"))
}

async fn delete_task_handler(State(todo_app): State<TodoApp>, Path(id): Path<u32>) -> Html<String> {
    todo_app.delete(id).unwrap();
    Html(format!("<h2>Task #{id} is deleted !</h2>"))
}

async fn mark_task_handler(
    State(todo_app): State<TodoApp>,
    Path(id): Path<u32>,
    Json(is_ready): Json<bool>,
) -> Html<String> {
    todo_app.mark_ready_or_not(id, is_ready).unwrap();
    Html(format!(
        "<h2>Task #{id} is marked as {} !</h2>",
        ready_or_not(is_ready)
    ))
}

async fn handler() -> Html<String> {
    let main_tip = String::from("<h1>Hello, friend!</h1>\n");
    let read_one_tip = String::from("1) To read task -> /read/{id}\n");
    Html(format!("{main_tip} {read_one_tip}"))
}
