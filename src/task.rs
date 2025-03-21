use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn get_completion(&self) -> &bool {
        &self.completion
    }

    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    pub fn check(&mut self) {
        self.completion = true;
    }

}
