mod utils;
mod task;

use utils::{ collect_arguments, analyse_arguments };
use task::Task;


fn main() {
    let arg_list = collect_arguments();
    analyse_arguments(&arg_list);
}
