use chrono::Local;
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "liznote")]
#[command(about = "A cross-platform CLI todo list application", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add {
        /// Task description
        #[arg(value_name = "TASK")]
        task: String,
    },
    /// List all todos
    List {
        /// Show only completed tasks
        #[arg(short, long)]
        done: bool,
        /// Show only pending tasks
        #[arg(short, long)]
        pending: bool,
    },
    /// Mark a todo as done
    Done {
        /// Task index (1-based)
        #[arg(value_name = "ID")]
        id: usize,
    },
    /// Remove a todo
    Remove {
        /// Task index (1-based)
        #[arg(value_name = "ID")]
        id: usize,
    },
    /// Clear all todos
    Clear,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: usize,
    task: String,
    completed: bool,
    created_at: String,
}

struct TodoList {
    todos: Vec<Todo>,
    data_file: PathBuf,
}

impl TodoList {
    fn new() -> Self {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("liznote");
        
        let _ = fs::create_dir_all(&data_dir);
        let data_file = data_dir.join("todos.json");
        
        let todos = if data_file.exists() {
            fs::read_to_string(&data_file)
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        
        TodoList { todos, data_file }
    }
    
    fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.todos) {
            let _ = fs::write(&self.data_file, json);
        }
    }
    
    fn add(&mut self, task: String) {
        let id = self.todos.len() + 1;
        let todo = Todo {
            id,
            task,
            completed: false,
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        self.todos.push(todo);
        self.save();
    }
    
    fn list(&self, filter: Option<&str>) {
        if self.todos.is_empty() {
            println!("{}", "No todos yet! Add one with: liznote add <task>".yellow());
            return;
        }
        
        println!("\n{}", "═══════════════════════════════════════════════════════════".bold());
        println!("{}", format!("{:<4} {:<40} {:<15}", "ID", "Task", "Status").bold());
        println!("{}", "═══════════════════════════════════════════════════════════".bold());
        
        for todo in &self.todos {
            let should_show = match filter {
                Some("done") => todo.completed,
                Some("pending") => !todo.completed,
                _ => true,
            };
            
            if should_show {
                let status = if todo.completed {
                    "✓ Done".green()
                } else {
                    "⏳ Pending".yellow()
                };
                
                let task_display = if todo.completed {
                    todo.task.strikethrough()
                } else {
                    todo.task.normal()
                };
                
                println!("{:<4} {:<40} {}", todo.id, task_display, status);
            }
        }
        
        println!("{}\n", "═══════════════════════════════════════════════════════════".bold());
    }
    
    fn done(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            self.save();
            println!("{}", format!("✓ Task {} marked as done!", id).green().bold());
        } else {
            println!("{}", format!("✗ Task {} not found!", id).red().bold());
        }
    }
    
    fn remove(&mut self, id: usize) {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            let task = &self.todos[pos].task;
            self.todos.remove(pos);
            self.save();
            println!("{}", format!("✓ Removed: '{}'", task).green().bold());
        } else {
            println!("{}", format!("✗ Task {} not found!", id).red().bold());
        }
    }
    
    fn clear(&mut self) {
        if self.todos.is_empty() {
            println!("{}", "No todos to clear!".yellow());
            return;
        }
        
        self.todos.clear();
        self.save();
        println!("{}", "✓ All todos cleared!".green().bold());
    }
}

fn main() {
    let cli = Cli::parse();
    let mut todolist = TodoList::new();
    
    match cli.command {
        None => {
            // Show help if no command
            todolist.list(None);
        }
        Some(Commands::Add { task }) => {
            todolist.add(task);
            println!("{}", "✓ Task added!".green().bold());
        }
        Some(Commands::List { done, pending }) => {
            let filter = if done { Some("done") } else if pending { Some("pending") } else { None };
            todolist.list(filter);
        }
        Some(Commands::Done { id }) => {
            todolist.done(id);
        }
        Some(Commands::Remove { id }) => {
            todolist.remove(id);
        }
        Some(Commands::Clear) => {
            todolist.clear();
        }
    }
}