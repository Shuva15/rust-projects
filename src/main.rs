mod handlers;
mod models;
mod state;

use axum::{routing::{get, post}, Router};

use handlers::{show_todos, add_todo_handler, complete_todo_handler, delete_completed_todos_handler};
use state::AppState;
use std::net::SocketAddr;
use dotenvy::dotenv;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = SqlitePool::connect("sqlite:todos.db").await.expect("Failed to connect to database");

    // Initialize the table if it doesn't exist
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            todo TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to initialize database");

    let app_state = AppState {
        db: pool
    };

    let app = Router::new()
        .route("/", get(show_todos))
        .route("/add", post(add_todo_handler))
        .route("/complete/{id}", post(complete_todo_handler))
        .route("/delete", post(delete_completed_todos_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
