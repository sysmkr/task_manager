// This file contains used tools in 'src/main.rs' to handle and process arguments

use std::env;

// Gather arguments from "cargo run -- args"

pub fn collect_arguments() -> Vec<String> {
    let mut arg_list: Vec<String> = env::args().collect(); // Collect arguments
    arg_list.remove(0); // Remove "target/debug/package_name"
    arg_list // Return cleaned argument list
}

// Compare what has been provided in "cargo run -- args"

pub fn analyse_arguments(arg_list: &Vec<String>) {

    match arg_list.len() {
        0 => println!("[ERROR] No argument provided, use : cargo run -- argument"),
        _ => match arg_list[0].trim() {

            "add" => println!("Adding..."),
            "remove" => println!("Removing..."),
            "check" => println!("Checking..."),
            _ => println!("[ERROR] Enter a valid argument"),

        }
    }

}
