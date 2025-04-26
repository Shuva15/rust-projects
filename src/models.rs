use serde::{Deserialize, Serialize};
use askama::Template;

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: usize,
    pub todo: String,
    pub completed: bool,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct TodoListTemplate<'a> {
    pub todos: &'a Vec<TodoItem>,
}

