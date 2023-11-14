use std::{fs::{OpenOptions, File}, io::{Write, SeekFrom, Read, Seek}};
use console::{Term, Style};
use serde::{Deserialize, Serialize};

static TASKS_FILE: &'static str = "./src/tasks.json";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    details: String,
    finished: bool,
    priority: Priority 
}

impl Task {
    pub fn new(details: String, finished: bool, priority: Priority) -> Self {
        Task {
            details,
            finished,
            priority,
        }
    }

    pub fn get_details(&self) -> &String {
        &self.details
    }

    pub fn get_finished(&self) -> &bool {
        &self.finished
    }

    pub fn get_priority(&self) -> &Priority {
        &self.priority
    }

    pub fn set_finished(&mut self, finished: bool) {
        self.finished = finished;
    }

    pub fn set_details(&mut self, details: String) {
        self.details = details;
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Priority {
    LOW,
    MEDIUM,
    HIGH,
}

impl Priority {
    pub fn to_usize(&self) -> usize {
        match self {
            Priority::LOW => 0,
            Priority::MEDIUM => 1,
            Priority::HIGH => 2,
        }
    }
}

pub fn read_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().read(true).write(true).open(TASKS_FILE)?;

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let tasks: Vec<Task> = serde_json::from_str(&content)?;

    Ok(tasks)
}

pub fn add_task(task: Task) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = read_tasks()?; 

    tasks.push(task);

    let mut file = File::create(TASKS_FILE)?;
    file.seek(SeekFrom::Start(0))?;

    let updated_content = serde_json::to_string_pretty(&tasks)?;

    file.write_all(updated_content.as_bytes())?;

    Ok(())
}

pub fn delete_task(index: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = read_tasks()?;

    let mut file = File::create(TASKS_FILE)?;
    file.seek(SeekFrom::Start(0))?;

    if index < tasks.len() {
        tasks.remove(index);

        let updated_content = serde_json::to_string_pretty(&tasks)?;

        file.write_all(updated_content.as_bytes())?;
    } else {
        println!("Index out of bounds");
    }

    Ok(())
}

pub fn edit_task(index: usize, new_task: Task) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = read_tasks()?;

    let mut file = File::create(TASKS_FILE)?;
    file.seek(SeekFrom::Start(0))?;

    if index < tasks.len() {
        tasks[index] = new_task;

        let updated_content = serde_json::to_string_pretty(&tasks)?;
        file.write_all(updated_content.as_bytes())?;
    } else {
        println!("Index out of bounds");
    }
    Ok(())
}

pub fn update_tasks(tasks: Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(TASKS_FILE)?;
    file.seek(SeekFrom::Start(0))?;

    file.write_all(serde_json::to_string_pretty(&tasks)?.as_bytes())?;

    Ok(())
}

pub fn reset() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(TASKS_FILE)?;

    file.set_len(0)?;

    Ok(())
}

pub fn show_task() {
    let term = Term::stdout();
    let tasks = read_tasks().unwrap();

    for (index, task_element) in tasks.iter().enumerate() {
        let task_text = String::from(
            format!(
                "{}. [{}] {} | {}",
                index,
                match task_element.finished {
                    true => "x",
                    false => " ",
                },
                match task_element.get_priority() {
                    crate::task::Priority::LOW => Style::new().bold().color256(25).apply_to("LOW"),
                    crate::task::Priority::MEDIUM => Style::new().bold().color256(44).apply_to("MID"),
                    crate::task::Priority::HIGH => Style::new().bold().color256(88).apply_to("HI ")
                },
                task_element.details
            ));
        term.write_line(&task_text).unwrap();
    }
}
