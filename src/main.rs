use std::{fs::{File, self}, io::BufReader};

use serde::Deserialize;

static TASKS_FILE: &'static str = "./src/tasks.json";

#[derive(Debug, Deserialize)]
struct Task {
    details: String,
    finished: bool,
    priority: Priority 
}

#[derive(Debug, Deserialize)]
enum Priority {
    LOW,
    MEDIUM,
    HIGH,
}


fn read_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let file = File::open(TASKS_FILE)?;
    let reader = BufReader::new(file);
    
    let tasks: Vec<Task> = serde_json::from_reader(reader)?;

    Ok(tasks)
}

fn main() {
    println!("Tasks read: {:?}", read_tasks().unwrap());
}
