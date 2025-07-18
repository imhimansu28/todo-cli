mod todo;

use crate::todo::Todo;

enum Action {
    AddTask,
    ViewTasks,
    UpdateTask,
    DeleteTask,
    MarkTaskAsCompleted,
    Exit,
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn print_header(header: &str) {
    println!("---------{}---------", header);
}

fn choose_action(todo: &mut Todo) {
    let action_array = [
        "1. Add task",
        "2. View tasks",
        "3. Update task",
        "4. Delete task",
        "5. Mark task as completed",
        "6. Exit",
    ];

    let action_map = [
        Action::AddTask,
        Action::ViewTasks,
        Action::UpdateTask,
        Action::DeleteTask,
        Action::MarkTaskAsCompleted,
        Action::Exit,
    ];
    if action_array.len() != action_map.len() {
        println!("Error: action_array and action_map lengths do not match");
        return;
    }
    print_header("Choose an action");
    for action in action_array {
        println!("{}", action);
    }

    loop {
        let input = get_input("Enter your choice: ");
        let input: usize = match input.trim().parse() {
            Ok(num) if num >= 1 && num <= action_map.len() => num,
            _ => {
                print_header("Choose a valid action");
                for action in action_array {
                    println!("{}", action);
                }
                continue;
            }
        };

        match action_map[input - 1] {
            Action::AddTask => {
                print_header("Add Task");
                let name = get_input("Enter task name: ");
                let description = get_input("Enter task description: ");
                todo.add_task(name, description);
                print_header("Add Task Close");
            }
            Action::ViewTasks => {
                print_header("Task List");
                todo.view_tasks();
                print_header("Task List Close");
            }
            Action::UpdateTask => {
                print_header("Update Task");
                let id: u32 = get_input("Enter task id: ")
                    .parse()
                    .expect("Please enter a valid number");
                let name = get_input("Enter updated task name: ");
                let description = get_input("Enter updated task description: ");
                todo.update_task(id, name, description);
                print_header("Update Task Close");
            }
            Action::DeleteTask => {
                print_header("Delete Task");
                let id: u32 = get_input("Enter task id: ")
                    .parse()
                    .expect("Please enter a valid number");
                
                if todo.delete_task(id) {
                    println!("Task deleted successfully!");
                } else {
                    // Check if task exists but is not completed
                    if todo.tasks.iter().any(|t| t.id == id) {
                        println!("Task found but not completed. Only completed tasks can be deleted.");
                    } else {
                        println!("Task not found!");
                    }
                }
                print_header("Delete Task Close");
            }
            Action::MarkTaskAsCompleted => {
                print_header("Mark Task as Completed");
                let id: u32 = get_input("Enter task id: ")
                    .parse()
                    .expect("Please enter a valid number");
                
                if todo.is_completed(id) {
                    println!("Task is already completed!");
                } else if todo.mark_task_completed(id) {
                    println!("Task marked as completed successfully!");
                } else {
                    println!("Task not found!");
                }
                print_header("Mark Task as Completed Close");
            }
            Action::Exit => break,
        }
    }
}

fn main() {
    let mut todo = Todo::new();
    choose_action(&mut todo);
}
