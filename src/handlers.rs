use std::collections::HashMap;
use askama::Template;
use axum::{extract::{Path, State}, response::{Html, IntoResponse, Redirect}, Form};
use crate::{models::{TodoItem, TodoListTemplate}, state::AppState};

pub async fn add_todo_handler(
    State(state): State<AppState>,
    Form(form): Form<HashMap<String, String>>,
) -> Redirect {
    if let Some(todo_text) = form.get("todo") {
        let mut todos = state.todos.lock().unwrap();
        let id = todos.len() + 1;
        todos.push(
            TodoItem { id, todo: todo_text.clone(), completed: false }
        );
    };

    Redirect::to("/")
}

pub async fn complete_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> Redirect {
    let mut todos = state.todos.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = true;
        Redirect::to("/")
    } else {
        Redirect::to("/")
    }
}

pub async fn delete_completed_todos_handler(
    State(state): State<AppState>
) -> Redirect {
    let mut todos = state.todos.lock().unwrap();
    todos.retain(|todo| !todo.completed);
    for (index, todo) in todos.iter_mut().enumerate() {
        todo.id = index + 1;
    }
    Redirect::to("/")
}

pub async fn show_todos(State(state): State<AppState>) -> impl IntoResponse {
    let todos = state.todos.lock().unwrap();
    let template = TodoListTemplate { todos: &todos };
    Html(template.render().unwrap())
}