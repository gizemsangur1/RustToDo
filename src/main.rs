use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "Tasker")]
#[command(about = "Basit bir görev yöneticisi", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { title } => {
            let mut tasks = load_tasks();

            let id = if let Some(last) = tasks.last() {
                last.id + 1
            } else {
                1
            };

            let task = Task {
                id,
                title: title.clone(),
                done: false,
            };

            tasks.push(task);
            save_tasks(&tasks);

            println!("Görev eklendi: {}", title);
        }
    }
}

fn load_tasks() -> Vec<Task> {
    let path = Path::new("tasks.json");
    if !path.exists() {
        return vec![];
    }

    let mut file = File::open(path).expect("Görev dosyası açılamadı.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
}

fn save_tasks(tasks: &[Task]) {
    let mut file = File::create("tasks.json").expect("Görev dosyası yazılamadı.");
    let json = serde_json::to_string_pretty(tasks).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
