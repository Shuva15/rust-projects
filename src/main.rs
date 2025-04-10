use std::fs;
use std::io;
use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    #[derive(Deserialize, Serialize)]
    struct TodoItem {
        id: usize,
        todo: String,
        completed: bool
    }

    let commands = ["1. List all Todos", "2. Add a Todo", "3. Exit"];
    let mut user_input = String::new();

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

    for _ in 0..100 {
        // print all the command
        for com in commands {
            println!("{}", com);
        }
        println!("Enter a command: (1 or 2 or 3)");
        user_input.clear();
        io::stdin().read_line(&mut user_input).expect("Couldn't read the input.");

        if user_input.trim() == "1" {
            for todo in &all_todos {
                let complete_mark = if todo.completed {"✅"} else {"❌"};
                println!("{}.  {}  ->  {}", todo.id, todo.todo, complete_mark)
            };
        } else if user_input.trim() == "2" {
            // get the todo from user
            let mut add_todo = String::new();
            println!("Add a Todo: ");
            io::stdin().read_line(&mut add_todo).expect("Couldn't read the input.");
            // add the todo to the json file
            let todo_struct: TodoItem = TodoItem { id: all_todos.len() + 1, todo: add_todo.trim().to_string(), completed: false };
            all_todos.push(todo_struct);
            let todo_json = serde_json::to_string(&all_todos).expect("Couldn't convert to string");
            let _ = fs::write("todos.json", todo_json);
            println!("Todo Added.")
        } else if user_input.trim() == "3" {
           return; 
        } else {
            println!("Invalid Input! (Type 1 or 2 or 3)")
        }
    }
}