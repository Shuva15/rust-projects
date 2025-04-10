use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;
use std::usize;

#[derive(Deserialize, Serialize)]
struct TodoItem {
    id: usize,
    todo: String,
    completed: bool,
}

enum Commands {
    Add,
    List,
    Done,
    Remove,
    New,
    Exit,
    Invalid,
}

impl Commands {
    fn from_input(input: &str) -> Commands {
        match input {
            "a" => Commands::Add,
            "l" => Commands::List,
            "d" => Commands::Done,
            "r" => Commands::Remove,
            "n" => Commands::New,
            "e" => Commands::Exit,
            _ => Commands::Invalid,
        }
    }
}

fn main() {
    // get all the todos from json file
    let mut all_todos: Vec<TodoItem> = match fs::read_to_string("todos.json") {
        Ok(content) => {
            if content.trim().is_empty() {
                vec![]
            } else {
                serde_json::from_str(&content).expect("⚠️ Could not parse todos.json.")
            }
        }
        Err(_) => {
            println!("📄 File not found, starting with empty todo list.");
            vec![]
        }
    };

    loop {
        // get command input from user
        let input = get_user_input();
        let command = Commands::from_input(&input);

        match command {
            Commands::Add => add_todo(&mut all_todos),
            Commands::List => list_todos(&all_todos),
            Commands::Done => mark_todo_done(&mut all_todos),
            Commands::Remove => {
                all_todos.retain(|todo| !todo.completed);
                println!("Removed all the completed todos.")
            }
            Commands::New => start_new(&mut all_todos),
            Commands::Exit => break,
            Commands::Invalid => println!("❌ Invalid input, try again."),
        }
    }
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Couldn't read the input.");
    user_input.trim().to_lowercase()
}

fn add_todo(all_todos: &mut Vec<TodoItem>) {
    // get the todo from user
    let mut add_todo = String::new();
    println!("Add a Todo: ");
    io::stdin()
        .read_line(&mut add_todo)
        .expect("Couldn't read the input.");
    // add the todo to the json file
    let todo_struct: TodoItem = TodoItem {
        id: all_todos.len() + 1,
        todo: add_todo.trim().to_string(),
        completed: false,
    };
    all_todos.push(todo_struct);
    let todo_json = serde_json::to_string(&all_todos).expect("Couldn't convert to string");
    let _ = fs::write("todos.json", todo_json);
    println!("Todo Added.");
}

fn list_todos(all_todos: &Vec<TodoItem>) {
    for todo in all_todos {
        let complete_mark = if todo.completed { "✅" } else { "❌" };
        println!("{}.  {}  ->  {}", todo.id, todo.todo, complete_mark);
    }
}

fn mark_todo_done(all_todos: &mut Vec<TodoItem>) {
    let mut input = String::new();
    println!("Type the Todo index that you have completed:");
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read the input!");

    match input.trim().parse::<usize>() {
        Ok(value) => {
            if value > 0 && all_todos.len() >= value {
                all_todos[value - 1].completed = true;
                let todo_json =
                    serde_json::to_string(&all_todos).expect("Couldn't convert to string");
                let _ = fs::write("todos.json", todo_json);
                println!("{} is completed.", all_todos[value - 1].todo);
            } else {
                println!("Wrong input! Enter the index of the todo you want to set completed.");
            }
        }
        Err(_) => {
            println!("Wrong input! Enter the index of the todo you want to set completed.");
        }
    }
}

fn start_new(all_todos: &mut Vec<TodoItem>) {
    all_todos.clear();
    let todo_json = serde_json::to_string(&all_todos).expect("Couldn't convert to string");
    let _ = fs::write("todos.json", todo_json);
    println!("Removed all the Todos, a new todo list!");
}
