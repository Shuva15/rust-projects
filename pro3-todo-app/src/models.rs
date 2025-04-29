use askama::Template;

#[derive(Debug, sqlx::FromRow)]
pub struct TodoFromDb {
    pub id: i64,
    pub todo: String,
    pub completed: bool,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct TodoListTemplate<'a> {
    pub todos: &'a Vec<TodoFromDb>,
}

