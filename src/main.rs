use std::{fs::{OpenOptions, File}, io::{Write, SeekFrom, Read, Seek}, thread, time::Duration};

use serde::{Deserialize, Serialize};

static TASKS_FILE: &'static str = "./src/tasks.json";

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    details: String,
    finished: bool,
    priority: Priority 
}

#[derive(Debug, Deserialize, Serialize)]
enum Priority {
    LOW,
    MEDIUM,
    HIGH,
}


fn read_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().read(true).write(true).open(TASKS_FILE)?;

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let tasks: Vec<Task> = serde_json::from_str(&content)?;

    Ok(tasks)
}

fn add_task(task: Task) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = read_tasks()?; 

    tasks.push(task);

    let mut file = File::create(TASKS_FILE)?;
    file.seek(SeekFrom::Start(0))?;

    let updated_content = serde_json::to_string_pretty(&tasks)?;

    file.write_all(updated_content.as_bytes())?;

    Ok(())
}

fn delete_task(index: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().read(true).write(true).open(TASKS_FILE)?;

    let mut tasks = read_tasks()?;

    let mut file = File::create(TASKS_FILE)?;
    file.seek(SeekFrom::Start(0))?;

    if index < tasks.len() {
        tasks.remove(index);

        let updated_content = serde_json::to_string_pretty(&tasks)?;

        println!("{}", updated_content);

        file.write_all(updated_content.as_bytes())?;
    } else {
        println!("Index out of bounds");
    }

    Ok(())
}

fn mark_task(index: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().read(true).write(true).open(TASKS_FILE)?;

    let mut tasks = read_tasks()?;

    let mut file = File::create(TASKS_FILE)?;
    file.seek(SeekFrom::Start(0))?;

    if index < tasks.len() {
        tasks[index].finished = !tasks[index].finished;

        let updated_content = serde_json::to_string_pretty(&tasks)?;
        file.write_all(updated_content.as_bytes())?;
    } else {
        println!("Index out of bounds");
    }
    Ok(())
}

fn main() {
    let task = Task { 
        details: "Task added".to_owned(),
        finished: true,
        priority: Priority::MEDIUM
     };
    match add_task(task) {
        Ok(()) => println!("Task added successfully"),
        Err(err) => println!("Error: {}", err),
    }
    match delete_task(0) {
       Ok(()) => println!("Task deleted successfully"),
       Err(err) => println!("Error: {}", err),
    }
    match mark_task(1) {
        Ok(()) => println!("Task marked successfully"),
        Err(err) => println!("Error: {}", err),
    }
    println!("Tasks read: {:?}", read_tasks().unwrap());
}
