use std::env;
use crate::task::Task;

// List of utils
// collect_args() : Collect every passed argument and place it into a Vec<String>
// analyse_args() : Peform task using match arms by refering to the list of argument
// bring() : Transform JSON data into variables that will be used to perform modifications
// update() : Bring back data to JSON file from the same variables that may handled modifications

// Argument gathering from "cargo run -- <arg_list>"

pub fn collect_arguments() -> Vec<String> {
    let mut arg_list: Vec<String> = env::args().collect(); // Collect arguments
    arg_list.remove(0); // Remove "target/debug/package_name"
    arg_list // Return cleaned argument list
}

// Compare what has been provided in "cargo run -- <arg_list>"

pub fn analyse_arguments(arg_list: &Vec<String>, task_list: &mut Vec<Task>) {

    match arg_list.len() {
        0 => println!("[ERROR] No argument provided, use : cargo run -- argument"),
        _ => match arg_list[0].trim() {

            "add" => {
                let new_task: Task = Task::new(arg_list[1].to_string(), false);
                task_list.push(new_task);
                println!("[ADD] {:?}", task_list[task_list.len() - 1].get_title());
            }

            "remove" => println!("Removing..."),
            "check" => println!("Checking..."),
            _ => println!("[ERROR] Enter a valid argument"),

        }
    }

}
