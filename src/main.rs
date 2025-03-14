mod task;

use std::env;

// Tasky specific
use task::Task;

fn main() {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);

    match arguments[0].trim() {
        "add" => println!("add"),
        _ => println!("Invalid query"),
    }
}
