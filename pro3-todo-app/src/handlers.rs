use crate::{
    models::{TodoFromDb, TodoListTemplate},
    state::AppState,
};
use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
};
use sqlx;
use std::collections::HashMap;

pub async fn show_todos(State(state): State<AppState>) -> impl IntoResponse {
    let todos = sqlx::query_as!(TodoFromDb, "SELECT id, todo, completed FROM todos")
        .fetch_all(&state.db)
        .await
        .unwrap_or_else(|_| vec![]);

    let template = TodoListTemplate { todos: &todos };
    Html(template.render().unwrap())
}

pub async fn add_todo_handler(
    State(state): State<AppState>,
    Form(form): Form<HashMap<String, String>>,
) -> Redirect {
    if let Some(todo_text) = form.get("todo") {
        let trimmed_todo = todo_text.trim();
        // Access the pool from AppState
        let pool = &state.db;

        // Insert the new todo into the database
        if !trimmed_todo.is_empty() {
            sqlx::query!(
                "INSERT INTO todos (todo, completed) VALUES (?, ?)",
                todo_text,
                false
            )
            .execute(pool)
            .await
            .expect("Failed to insert new todo");
        }
    };

    Redirect::to("/")
}

pub async fn complete_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> Redirect {
    let pool = &state.db;
    let id_i32 = id as i32;
    // Update the completion status of the todo
    sqlx::query!("UPDATE todos SET completed = ? WHERE id = ?", true, id_i32)
        .execute(pool)
        .await
        .expect("Failed to complete todo");

    Redirect::to("/")
}

pub async fn delete_completed_todos_handler(State(state): State<AppState>) -> Redirect {
    let pool = &state.db;

    // Delete completed todos from the database
    sqlx::query!("DELETE FROM todos WHERE completed = ?", true)
        .execute(pool)
        .await
        .expect("Failed to delete completed todos");
    Redirect::to("/")
}
