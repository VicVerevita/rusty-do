use std::{time::Duration, thread};

use console::{Term, Style};
use dialoguer::{MultiSelect, Select, Input};

use crate::task::{Task, read_tasks, update_tasks, show_task, Priority, add_task, edit_task, delete_task, reset};

pub fn start() {
    let mut running = true;
    let term = Term::stdout();
    let _ = term.clear_screen();
    term.set_title("To Do List");
    let intro = String::from(format!(
        "{} {}{}\n{}", 
        Style::new().bold().color256(24).apply_to("Welcome to your CLI"), 
        Style::new().bold().color256(96).apply_to("To Do List"),
        Style::new().bold().color256(24).apply_to("!\n"),
        Style::new().bold().color256(24).apply_to("This is today's list:\n"),
    ));

    term.write_line(&intro).unwrap();
    thread::sleep(Duration::from_millis(500));    

    while running {
        let mut tasks: Vec<Task> = read_tasks().unwrap();
        let marked_tasks = show_tasks();

        for index in 0..tasks.len() {
            if marked_tasks.contains(&index) {
                tasks[index].set_finished(true);
            } else {
                tasks[index].set_finished(false);
            }
        }

        let _ = update_tasks(tasks.clone());

        show_task();

        running = select_actions();
        let _ = term.clear_screen();
    }

    let outro = String::from(format!(
        "{}",
        Style::new().bold().color256(24).apply_to("See you later!")
    ));

    term.write_line(&outro).unwrap();
}


fn show_tasks() -> Vec<usize> {
    let tasks = read_tasks().unwrap();
    let tasks_clone = tasks.clone();
    let formatted_tasks: Vec<String> = tasks_clone.into_iter().map(|task: Task| String::from(format!(
        "{} | {}", 
        match task.get_priority() {
            crate::task::Priority::LOW => Style::new().bold().color256(25).apply_to("LOW"),
            crate::task::Priority::MEDIUM => Style::new().bold().color256(44).apply_to("MID"),
            crate::task::Priority::HIGH => Style::new().bold().color256(88).apply_to("HI")
        },
        task.get_details()))).collect();

    let tasks_clone = tasks.clone();
    let tasks_prio: Vec<bool> = tasks_clone.into_iter().map(|task| *task.get_finished()).collect();
    let selection = MultiSelect::new().with_prompt("Update the status of the tasks. Press enter when you finish").report(false).items(&formatted_tasks).defaults(&tasks_prio).interact().unwrap();

    selection
}

fn select_actions() -> bool {
    let tasks = read_tasks().unwrap();
    let items = vec!["Add", "Edit", "Delete", "Reset", "Exit"];

    let selection = Select::new().with_prompt(format!("{}", Style::new().bold().color256(24).apply_to("Select the action you want to take now"))).items(&items).default(0).interact().unwrap();

    match selection {
        0 => {
            let details = Input::<String>::new().with_prompt("Write the task you want to add to the list").interact_text().unwrap();
            let finished_options = vec!["Done", "Not Done"];
            let finished = Select::new().with_prompt("Select the status of the task").items(&finished_options).default(0).interact().unwrap();
            let finished = match finished {
                0 => true,
                1 => false,
                _ => false,
            };
            let priority_options = vec!["Low", "Mid", "High"];
            let priority = Select::new().with_prompt("Select the priority of the task").items(&priority_options).default(0).interact().unwrap();
            let priority = match priority {
                0 => Priority::LOW,
                1 => Priority::MEDIUM,
                2 => Priority::HIGH,
                _ => Priority::LOW
            };
            let new_task = Task::new(details, finished, priority);
            add_task(new_task).unwrap();
            true
        }
        1 => {
            let task_index = Input::<String>::new().with_prompt("Input the number of the task you want to edit").interact_text().unwrap();
            let task_index: usize = task_index.parse().unwrap();
            let modified_task = tasks[task_index].clone();
            let details = Input::<String>::new().with_prompt("Modify the task details").with_initial_text(modified_task.get_details()).interact_text().unwrap();
            let finished_options = vec!["Done", "Not Done"];
            let finished_default = modified_task.get_finished();
            let finished = Select::new().with_prompt("Select the status of the task").items(&finished_options).default(!finished_default as usize).interact().unwrap();
            let finished = match finished {
                0 => true,
                1 => false,
                _ => false,
            };
            let priority_options = vec!["Low", "Mid", "High"];
            let priority_default = modified_task.get_priority();
            let priority = Select::new().with_prompt("Select the priority of the task").items(&priority_options).default(priority_default.to_usize()).interact().unwrap();
            let priority = match priority {
                0 => Priority::LOW,
                1 => Priority::MEDIUM,
                2 => Priority::HIGH,
                _ => Priority::LOW
            }; 
            let modified_task = Task::new(details, finished, priority);
            edit_task(task_index, modified_task).unwrap();
            true
        }
        2 => {
            let task_index = Input::<String>::new().with_prompt("Input the number of the task you want to edit").interact_text().unwrap();
            let task_index: usize = task_index.parse().unwrap();
            delete_task(task_index).unwrap();
            true
        }
        3 => {
            let _ = reset();
            true
        }
        4 => {
            false
        }
        _ => { 
            println!("Invalid option!");
            true
        }
    }
}

