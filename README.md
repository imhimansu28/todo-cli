# Rust Todo CLI

This is a command-line interface (CLI) application for managing todo tasks, written in Rust.

## Features

- Add new tasks with a title and description
- View a list of all tasks
- Update task title and description
- Delete completed tasks
- Mark tasks as completed

## Usage

1. Clone the repository:

   ```
   git clone https://github.com/your-username/rust-todo-cli.git
   ```

2. Navigate to the project directory:

   ```
   cd rust-todo-cli
   ```

3. Build the project:

   ```
   cargo build --release
   ```

4. Run the application:

   ```
   ./target/release/rust-todo-cli
   ```

5. Follow the on-screen prompts to interact with the todo application.

## Dependencies

- `serde` for serialization and deserialization of task data
- `serde_json` for JSON serialization and deserialization
- `std::fs` for file I/O operations
- `std::io` for input/output operations

## File Structure

- `src/main.rs`: Contains the main application logic and user interaction
- `src/todo.rs`: Defines the `Task` and `Todo` structs and their associated methods
- `tasks.json`: Stores the task data in JSON format

## Acknowledgements

- The Rust programming language and its ecosystem
- The `serde` and `serde_json` crates for easy serialization and deserialization
