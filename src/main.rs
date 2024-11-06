mod cli;
mod fs;

use clap::Parser;
use clap::CommandFactory;
use cli::{TodoArgs, TodoCommands};
use colored::*;
use fs::{complete_task, delete_task, get_task_ids, print_tasks, save_task};
use rustyline::DefaultEditor;

fn prompt_for_name(action: &str, reader: &mut DefaultEditor) -> String {
    loop {
        let prompt = format!("{}: ", format!("Enter task to {}", action).bright_green().bold());
        match reader.readline(&prompt) {
            Ok(input) => {
                let task = input.trim().to_string();
                if !task.is_empty() {
                    break task;
                } else {
                    eprintln!("{}", "Invalid task.".red());
                    continue;
                }
            }
            Err(_) => continue,
        }
    }
}

fn prompt_for_desc(reader: &mut DefaultEditor) -> Option<String> {
    loop {
        let prompt = "Add a description to this task? (Y/n): ".to_string();
        match reader.readline(&prompt) {
            Ok(input) => {
                let response = input.trim().to_uppercase();
                if response == "Y" || response.is_empty() {
                    let prompt = format!("{}: ", "Enter description".bright_green().bold());
                    match reader.readline(&prompt) {
                        Ok(desc) => break Some(desc),
                        Err(_) => continue,
                    }
                } else if response == "N" {
                    break None;
                } else {
                    eprintln!("{}", "Please enter 'Y' or 'N'.".red());
                }
            }
            Err(_) => continue,
        }
    }
}

fn get_task_data(
    action: &str,
    name: Option<String>,
    description: Option<String>,
    reader: &mut DefaultEditor,
    prompt_for_description: bool,
) -> (String, Option<String>) {
    let name = name.unwrap_or_else(|| prompt_for_name(action, reader));
    let description = if prompt_for_description {
        description.or_else(|| prompt_for_desc(reader))
    } else {
        description
    };
    println!();
    (name, description)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = DefaultEditor::new()?;
    let args = TodoArgs::parse();

    if let Some(command) = args.command {
        match command {
            TodoCommands::Add { name, description } => {
                let (name, desc) = get_task_data("add", name, description, &mut reader, true);
                save_task(name, desc)?;
            }
            TodoCommands::Complete { name } => {
                let (name, _) = get_task_data("complete", name, None, &mut reader, false);
                let ids = get_task_ids(&name)?;

                if ids.is_empty() {
                    println!(
                        "{}: {}",
                        "Found no tasks with name".bright_purple().bold(),
                        name
                    );
                } else {
                    complete_task(ids[0])?;
                }
            }
            TodoCommands::Delete { name } => {
                let (name, _) = get_task_data("delete", name, None, &mut reader, false);
                let ids = get_task_ids(&name)?;

                if ids.is_empty() {
                    println!(
                        "{}: {}",
                        "Found no tasks with name".bright_purple().bold(),
                        name
                    );
                } else {
                    delete_task(ids[0])?;
                }
            }
            TodoCommands::AllTasks => {
                print_tasks()?;
            }
        }
    } else {
        // If no command is provided, print help information
        println!("{}", TodoArgs::command().render_long_help());
    }

    Ok(())
}