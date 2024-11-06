use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum TodoCommands {
    /// Add a new task
    Add {
        #[clap(short, long)]
        name: Option<String>,

        #[clap(short, long)]
        description: Option<String>,
    },

    /// Complete a task
    Complete {
        #[clap(short, long)]
        name: Option<String>,
    },

    /// Delete a task
    Delete {
        #[clap(short, long)]
        name: Option<String>,
    },

    /// List all tasks
    AllTasks,
}

#[derive(Parser)]
#[clap(author, version, about = "A simple CLI task manager")]
pub struct TodoArgs {
    #[clap(subcommand)]
    pub command: Option<TodoCommands>,
}
