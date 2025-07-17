use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Error, ErrorKind};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            completed: false,
        }
    }

    pub fn print(&self) {
        let status = if self.completed { "✓" } else { "○" };
        println!("{} {} - {} - {}", status, self.id, self.title, self.description);
    }

    pub fn update(&mut self, title: String, description: String) {
        self.title = title;
        self.description = description;
    }

    pub fn set_completed(&mut self) {
        self.completed = true;
    }
}

pub struct Todo {
    pub tasks: Vec<Task>,
    next_id: u32,
}

impl Todo {
    pub fn new() -> Self {
        let tasks = match File::open("tasks.json") {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap_or_else(|_| Vec::<Task>::new())
            }
            Err(ref error) if error.kind() == ErrorKind::NotFound => Vec::<Task>::new(),
            Err(error) => panic!("An error occurred: {:?}", error),
        };

        let next_id = match tasks.last() {
            Some(task) => task.id + 1,
            None => 1,
        };

        Todo { tasks, next_id }
    }

    pub fn add_task(&mut self, title: String, description: String) {
        let task = Task::new(self.next_id, title, description);
        self.tasks.push(task);
        self.next_id += 1;
        self.save_tasks().expect("Failed to save tasks");
    }

    pub fn view_tasks(&self) {
        for task in &self.tasks {
            task.print();
        }
    }

    pub fn update_task(&mut self, id: u32, title: String, description: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.update(title, description);
            self.save_tasks().expect("Failed to save tasks");
        } else {
            println!("Task not found");
        }
    }

    pub fn delete_task(&mut self, id: u32) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            let task = &self.tasks[index];
            if task.completed {
                self.tasks.remove(index);
                self.save_tasks().expect("Failed to save tasks");
                true
            } else {
                false // Task exists but is not completed
            }
        } else {
            false // Task not found
        }
    }

    pub fn is_completed(&self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter().find(|t| t.id == id) {
            task.completed
        } else {
            false
        }
    }

    pub fn mark_task_completed(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.set_completed();
            self.save_tasks().expect("Failed to save tasks");
            true
        } else {
            false
        }
    }

    fn save_tasks(&self) -> Result<(), Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("tasks.json")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.tasks)?;
        Ok(())
    }
}
