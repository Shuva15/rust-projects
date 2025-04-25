mod handlers;
mod models;
mod state;

use axum::{routing::{get, post}, Router};

use handlers::{show_todos, add_todo_handler, complete_todo_handler, delete_completed_todos_handler};
use models::TodoItem;
use state::AppState;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() {
    // Starting with some dummy todos
    let initial_todos = vec![
        TodoItem {
            id: 1,
            todo: "Learn Rust".to_string(),
            completed: false,
        },
        TodoItem {
            id: 2,
            todo: "Build a web app".to_string(),
            completed: false,
        },
    ];

    let app_state = AppState {
        todos: Arc::new(Mutex::new(initial_todos)),
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
