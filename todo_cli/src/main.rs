use std::{fs, path::Path};
use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

const DB_FILE: &str = "todo.json";

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u64,
    text: String,
    done: bool,
}

#[derive(Parser, Debug)]
#[command(name = "todo", version, about = "Tiny CLI To-Do app in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task
    Add { text: String },

    /// List all tasks
    List,

    /// Mark a task as done by id
    Done { id: u64 },

    /// Remove a task by id
    Remove { id: u64 },

    /// Clear all tasks
    Clear,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut tasks = load_tasks()?;

    match cli.command {
        Commands::Add { text } => {
            let id = next_id(&tasks);
            tasks.push(Task { id, text: text.clone(), done: false });
            save_tasks(&tasks)?;
            println!("Added #{id}: {text}");
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks yet. Use `todo add \"Task...\"` to create one.");
            } else {
                for t in &tasks {
                    let box_mark = if t.done { "x" } else { " " };
                    println!("{:>4} [{}] {}", t.id, box_mark, t.text);
                }
            }
        }
        Commands::Done { id } => {
            if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                t.done = true;
                save_tasks(&tasks)?;
                println!("Marked #{id} as done.");
            } else {
                println!("No task found with id #{id}.");
            }
        }
        Commands::Remove { id } => {
            let before = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() < before {
                save_tasks(&tasks)?;
                println!("Removed #{id}.");
            } else {
                println!("No task found with id #{id}.");
            }
        }
        Commands::Clear => {
            tasks.clear();
            save_tasks(&tasks)?;
            println!("Cleared all tasks.");
        }
    }

    Ok(())
}

fn load_tasks() -> Result<Vec<Task>> {
    let path = Path::new(DB_FILE);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    let tasks = serde_json::from_str::<Vec<Task>>(&data)?;
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> Result<()> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(DB_FILE, data)?;
    Ok(())
}

fn next_id(tasks: &Vec<Task>) -> u64 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}
