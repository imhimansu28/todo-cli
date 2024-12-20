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
    print_header("Choose an action");
    for action in action_array {
        println!("{}", action);
    }

    loop {
        let input = get_input("Enter your choice: ");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print_header("Choose a valid action");
                for action in action_array {
                    println!("{}", action);
                }
                continue;
            }
        };

        match action_map[input as usize - 1] {
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
                todo.delete_task(id);
                print_header("Delete Task Close");
            }
            Action::MarkTaskAsCompleted => {
                print_header("Mark Task as Completed");
                let id: u32 = get_input("Enter task id: ")
                    .parse()
                    .expect("Please enter a valid number");
                let is_completed = todo.is_completed(id);
                println!("Task is completed: {}", is_completed);
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
