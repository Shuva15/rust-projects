use std::sync::{Arc, Mutex};

use crate::models::TodoItem;

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<Mutex<Vec<TodoItem>>>,
}