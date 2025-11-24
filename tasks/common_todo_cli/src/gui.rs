use anyhow::Ok;

use axum::{
    Form, Router,
    extract::{Path, State},
    response::{Html, Redirect},
    routing::{get, post},
};
use serde::Deserialize;
use std::sync::Arc;

use crate::{CommonTodo, db::AsyncStorage, model::task::Task, ready_or_not};

type TodoApp = Arc<dyn AsyncStorage + Send + Sync + 'static>;

pub async fn gui_mode<S>(todo_app: CommonTodo<S>) -> anyhow::Result<()>
where
    S: AsyncStorage + Send + Sync + 'static,
{
    println!("gui mode is active");

    let shared_state: Arc<dyn AsyncStorage + Send + Sync + 'static> = Arc::new(todo_app.storage);

    let app = Router::new()
        .route("/", get(handler))
        .route("/tasks", post(create_task_handler))
        .route("/tasks", get(read_all_tasks_handler))
        .route("/tasks/new", get(show_create_form_handler))
        .route("/tasks/{id}", get(read_task_by_id_handler))
        .route("/tasks/{id}/edit", get(show_edit_form))
        .route("/tasks/{id}/update", post(update_task_handler))
        .route("/tasks/{id}/delete", post(delete_task_handler))
        .route("/tasks/{id}/mark", post(mark_task_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn show_create_form_handler() -> Html<String> {
    Html(create_task_form())
}

async fn create_task_handler(
    State(todo_app): State<TodoApp>,
    Form(payload): Form<CreateTaskForm>,
) -> Redirect {
    todo_app.create(payload.value).await.unwrap();
    Redirect::to("/tasks?success=created")
}

async fn read_all_tasks_handler(State(todo_app): State<TodoApp>) -> Html<String> {
    let tasks = todo_app.list().await.unwrap();
    Html(view_all_tasks(tasks))
}

async fn read_task_by_id_handler(
    State(todo_app): State<TodoApp>,
    Path(id): Path<u32>,
) -> Html<String> {
    let task = todo_app.read(id).await.unwrap();
    Html(format!(
        "<h2>Your task #{} is {}: {}</h2>",
        task.id,
        ready_or_not(task.is_ready),
        task.value
    ))
}

async fn show_edit_form(State(todo_app): State<TodoApp>, Path(id): Path<u32>) -> Html<String> {
    let task = todo_app.read(id).await.unwrap();
    Html(edit_task_form(&task))
}

async fn update_task_handler(
    State(todo_app): State<TodoApp>,
    Path(id): Path<u32>,
    Form(payload): Form<UpdateTaskForm>,
) -> Redirect {
    todo_app.update(id, payload.value).await.unwrap();
    Redirect::to("/tasks?success=updated")
}

async fn delete_task_handler(State(todo_app): State<TodoApp>, Path(id): Path<u32>) -> Redirect {
    todo_app.delete(id).await.unwrap();
    Redirect::to("/tasks?success=deleted")
}

async fn mark_task_handler(State(todo_app): State<TodoApp>, Path(id): Path<u32>) -> Redirect {
    let task = todo_app.read(id).await.unwrap();
    let mark = !task.is_ready;
    todo_app.mark_ready_or_not(id, mark).await.unwrap();
    let path = format!("/tasks/{id}/edit");
    Redirect::to(&path)
}

async fn handler() -> Html<String> {
    let main_tip = String::from("<h1>Hello, friend!</h1>\n");
    Html(main_tip.to_string())
}

#[derive(Deserialize)]
pub struct CreateTaskForm {
    pub value: String,
}

#[derive(Deserialize)]
pub struct UpdateTaskForm {
    pub value: String,
}

fn layout(content: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>Todo App - Create Task</title>
</head>
<body>
    <div class="container">
        <h1>Todo App</h1>
        {content}
    </div>
</body>
</html>
    "#
    )
}

fn create_task_form() -> String {
    layout(&String::from(
        r#"
    <div class="form">
        <h2>Create New Task</h2>
        <form action="/tasks" method="post">
            <div>
                <label for="value">Task Description:</label>
                <textarea 
                    id="value" 
                    name="value" 
                    placeholder="Enter your task here..." 
                    rows="4" 
                    required
                    style="width: 100%; padding: 8px; margin: 5px 0;"
                ></textarea>
            </div>
            <br>
            <button type="submit" style="padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 4px;">
                Create Task
            </button>
            <a href="/tasks" style="margin-left: 10px;">Back to List</a>
        </form>
    </div>
    "#,
    ))
}

fn edit_task_form(task: &Task) -> String {
    let content = format!(
        r#"
<div class="form">
    <h2>Edit Task #{}{}</h2>

    <form action="/tasks/{}/update" method="post">
        <div class="form-group">
            <label for="value">Task Description:</label>
            <textarea 
                id="value" 
                name="value" 
                rows="4" 
                required
                style="width: 100%; padding: 12px; margin: 5px 0; border: 1px solid #ddd; border-radius: 4px; font-size: 16px;"
            >{}</textarea>
        </div>
        
        <div class="form-actions">
            <button type="submit" class="btn btn-primary">💾 Update Task</button>
            <a href="/tasks" class="btn btn-secondary">← Cancel</a>
        </div>
    </form>

    <div style="margin-top: 20px; padding-top: 20px; border-top: 1px solid #eee;">
        <h4>Quick Actions:</h4>
        <form action="/tasks/{}/mark" method="post" style="display: inline;">
            <button type="submit" class="btn btn-sm btn-warning">{}</button>
        </form>
        <form action="/tasks/{}/delete" method="post" style="display: inline;">
            <button type="submit" class="btn btn-danger">Delete</button>
        </form>
    </div>
</div>
"#,
        task.id,
        if task.is_ready {
            " ready"
        } else {
            " not ready"
        },
        task.id,
        task.value,
        task.id,
        if task.is_ready {
            "⏪ Mark Not Ready"
        } else {
            "✅ Mark Ready"
        },
        task.id
    );

    layout(&content)
}

fn view_all_tasks(tasks: Vec<Task>) -> String {
    let tasks_html: String = tasks
        .iter()
        .rev()
        .map(|task| {
            format!(
                r#"
            <div class="task">
                <a href="/tasks/{}/edit"><p>#{}: {}</p></a>
                <div>
                    <a href="/tasks/{}/edit">✏️ Edit</a>
                    <form action="/tasks/{}/delete" method="post" style="display: inline;">
                        <button type="submit" class="btn btn-danger">Delete</button>
                    </form>
                </div>
            </div>
            "#,
                task.id, task.id, task.value, task.id, task.id
            )
        })
        .collect();

    layout(&format!(
        r#"
    <div class="header">
        <a href="/tasks/new" class="btn">➕ Create New Task</a>
    </div>
    <div class="tasks">
        <h2>Your Tasks ({})</h2>
        {}
    </div>
    "#,
        tasks.len(),
        tasks_html
    ))
}
