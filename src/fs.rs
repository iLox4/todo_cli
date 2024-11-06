use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json;

const TASKS_FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u32,
    name: String,
    description: Option<String>,
    completed: bool,
}

pub fn get_task_ids(name: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let tasks = get_tasks()?;
    let name_lower = name.to_lowercase();

    let matching_ids: Vec<u32> = tasks
        .iter()
        .filter(|task| task.name.to_lowercase().contains(&name_lower))
        .map(|task| task.id)
        .collect();

    // Return matching IDs even if empty
    Ok(matching_ids)
}

/// Retrieves the list of tasks from the persistent storage.
pub fn get_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let mut file = match File::open(TASKS_FILE) {
        Ok(file) => file,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            // Return an empty list if the file doesn't exist
            return Ok(Vec::new());
        }
        Err(e) => return Err(Box::new(e)),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tasks: Vec<Task> = serde_json::from_str(&contents)?;
    Ok(tasks)
}

/// Saves the list of tasks to the persistent storage.
fn save_tasks(tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string(tasks)?;
    let mut file = File::create(TASKS_FILE)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

/// Adds a new task.
pub fn save_task(name: String, description: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = get_tasks()?;
    let id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    let task = Task {
        id,
        name,
        description,
        completed: false,
    };
    tasks.push(task);
    save_tasks(&tasks)?;
    Ok(())
}

/// Marks the task with the given ID as completed.
pub fn complete_task(id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = get_tasks()?;
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.completed = true;
        save_tasks(&tasks)?;
        Ok(())
    } else {
        Err("Task not found.".into())
    }
}

/// Deletes the task with the given ID.
pub fn delete_task(id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = get_tasks()?;
    let initial_len = tasks.len();
    tasks.retain(|task| task.id != id);
    if tasks.len() < initial_len {
        save_tasks(&tasks)?;
        Ok(())
    } else {
        Err("Task not found.".into())
    }
}

pub fn print_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let tasks = get_tasks()?;
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            let status = if task.completed { "[x]" } else { "[ ]" };
            match &task.description {
                Some(desc) => println!("{}: {} - {}", status, task.name, desc),
                None => println!("{}: {}", status, task.name),
            }
        }
    }
    Ok(())
}
