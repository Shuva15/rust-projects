# rust-projects
A collection of Rust-based projects built to learn and explore the Rust ecosystem.

This repo documents my Rust learning journey — building small, focused tools to explore different parts of the language, from CLIs to web apps.

## Projects

### 1. [`todo-cli`](./pro1-todo-cli)
A command-line todo app using local JSON storage.  
**Tech:** Rust, Serde

### 2. [`vault-entry`](./pro2-vault-entry)
A basic local password manager with AES encryption.  
**Tech:** Rust, aes-gcm, Serde, Rpassword

### 3. [`todo-web`](./pro3-todo-app)
A simple web-based todo app with SQLite and HTML templating.  
**Tech:** Rust, Axum, SQLx, Askama

## Getting Started

Each project lives in its own folder and includes its own `README.md`.  
To run any project:

```bash
cd <project-folder>
cargo run
