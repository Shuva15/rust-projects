use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{response::Html, extract::State, Json, http::StatusCode};
use serde::Deserialize;
use askama::Template;
use crate::{models::TodoItem, state::AppState};

#[derive(Deserialize)]
pub struct TodoInput {
    pub todo: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct TodoListTemplate<'a> {
    todos: &'a Vec<TodoItem>,
}

// pub async fn get_todos_handler(State(state): State<AppState>) -> Json<Vec<TodoItem>> {
//     let todos = state.todos.lock().unwrap();
//     Json(todos.clone())
// }

pub async fn add_todo_handler(
    State(state): State<AppState>,
    Json(payload): Json<TodoInput>,
) -> Json<TodoItem> {
    let mut todos = state.todos.lock().unwrap();

    let new_todo = TodoItem {
        id: todos.len() + 1,
        todo: payload.todo,
        completed: false,
    };

    todos.push(new_todo.clone());

    Json(new_todo)
}

pub async fn complete_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    let mut todos = state.todos.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = true;
        (
            StatusCode::OK,
            Json(format!("✅ Todo {} marked complete", id)),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(format!("❌ Todo with id {} not found", id)),
        )
    }
}

pub async fn delete_completed_todos_handler(
    State(state): State<AppState>
) {
    let mut todos = state.todos.lock().unwrap();
    todos.retain(|todo| !todo.completed);
}

pub async fn show_todos(State(state): State<AppState>) -> impl IntoResponse {
    let todos = state.todos.lock().unwrap();
    let template = TodoListTemplate { todos: &todos };
    Html(template.render().unwrap())
}