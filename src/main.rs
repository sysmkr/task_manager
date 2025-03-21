mod tools;
mod task;
use task::Task;
use std::path::Path;

fn main() {

    const data_file: &str = "./src/data.json";
    let mut task_list: Vec<Task> = tools::load_from(&data_file);
    let arg_list = tools::collect_args();
    tools::analyse_args(&arg_list, &mut task_list);
    tools::save_to(&data_file, &task_list);

}
