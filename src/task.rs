#[derive(Debug)]
pub struct Task {
    title: String,
    completion: bool,
}

impl Task {
    pub fn new(title: String, completion: bool) -> Self {
        Self { title, completion }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_completion(&self) -> bool {
        self.completion
    }

    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title
    }

    pub fn switch_completion(&mut self) {
        self.completion = !self.completion
    }
}

pub struct TaskList {
    list: Vec<Task>,
}

impl TaskList {
    pub fn print_list(&self) {
        for task in &self.list {
            println!("{:?}", task.get_title());
        }
    }
}
