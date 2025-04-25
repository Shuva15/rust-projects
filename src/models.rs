use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: usize,
    pub todo: String,
    pub completed: bool,
}

