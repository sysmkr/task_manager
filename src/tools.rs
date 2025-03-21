use std::env;
use crate::task::Task;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json;

// List of utils
// ----------------------------------------------------------------------------------------------
// collect_args() : Collect every passed argument and place it into a Vec<String>
// analyse_args() : Peform task using match arms by refering to the list of argument
// load_from() : Transform JSON data into variables that will be used to perform modifications
// save_to() : Bring back data to JSON file from the same variables that may handled modifications
// ----------------------------------------------------------------------------------------------

// Argument gathering from "cargo run -- <arg_list>"

pub fn collect_args() -> Vec<String> { // Validated
    let mut arg_list: Vec<String> = env::args().collect(); // Collect arguments
    arg_list.remove(0); // Remove "target/debug/package_name"
    arg_list // Return cleaned argument list
}

// Compare what has been provided in "cargo run -- <arg_list>"

pub fn analyse_args(arg_list: &Vec<String>, task_list: &mut Vec<Task>) {
    match arg_list.len() {
        0 => println!("[0 ARGUMENT]"),

        _ => match arg_list[0].trim() {
            "add" => {
                if arg_list.len() > 1 {
                    let new_task: Task = Task::new(arg_list[1].to_string(), false);
                    task_list.push(new_task);
                    println!("[ADD] '{:?}' added", task_list[task_list.len() - 1].get_title());
                } else {
                    println!("[ERROR] Task description missing");
                }
            }

            "remove" => {
                if arg_list.len() > 1 {
                    let task_title = arg_list[1].trim().to_string();
                    let position = task_list.iter().position(|task| *task.get_title() == task_title);
                    match position {
                        Some(index) => {
                            task_list.remove(index);
                            println!("[REMOVE] '{}' removed", task_title);
                        }
                        None => println!("[ERROR] No task with the title '{}' found", task_title),
                    }
                } else {
                    println!("[ERROR] Task description missing");
                }
            }

            "check" => {
                if arg_list.len() > 1 {
                    let task_title = arg_list[1].trim().to_string();
                    let mut searching_index: Option<usize> = None;

                    // Search for the task in task_list
                    for (index, item) in task_list.iter_mut().enumerate() {
                        if *item.get_title() == task_title {
                            searching_index = Some(index);
                            break;
                        }
                    }

                    match searching_index {
                        Some(index) => {
                            // Task found, set its completion to true
                            task_list[index].check();
                            println!("[CHECK] '{}' marked as completed", task_list[index].get_title());
                        }
                        None => {
                            println!("[ERROR] No task with the title '{}' found", task_title);
                        }
                    }
                } else {
                    println!("[ERROR] Task description missing");
                }
            }

            "list" => {

                println!("---");
                for item in task_list {
                    println!("{} : {}", item.get_title(), item.get_completion());
                }
                println!("---");

            }

            _ => println!("[ERROR] Enter a valid argument"),
        }
    }
}

// Transfer data into the program to interact with it

pub fn load_from(file_path: &str) -> Vec<Task> {
    let mut tasks = Vec::new();
    match File::open(file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Err(e) = file.read_to_string(&mut content) {
                eprintln!("Error reading from file: {}", e);
                return tasks;
            }
            match serde_json::from_str(&content) {
                Ok(loaded_tasks) => tasks = loaded_tasks,
                Err(e) => eprintln!("Error deserializing tasks from JSON: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error opening file {}: {}", file_path, e);
        }
    }
    tasks
}

// Write back the updated data into the JSON file

pub fn save_to(file_path: &str, task_list: &Vec<Task>) {
    match serde_json::to_string_pretty(task_list) {
        Ok(json_data) => {
            match File::create(file_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(json_data.as_bytes()) {
                        eprintln!("Error writing to file: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error creating file: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error serializing task list to JSON: {}", e);
        }
    }
}
