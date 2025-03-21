mod utils;
mod task;

use task::Task;

fn main() {

    // let data_file: &str = "data.json";
    let mut task_list: Vec<Task> = Vec::new();
    // utils::tansfer(&data_file);
    let arg_list = utils::collect_arguments();
    utils::analyse_arguments(&arg_list, &mut task_list);
    // utils::update(&data_file, &task_list)

}
