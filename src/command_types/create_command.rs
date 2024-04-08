use crate::command_trait::Command;
use crate::task_manager::TaskManager;

pub struct CreateCommand {
    params: Vec<String>,
}

impl CreateCommand {
    pub fn new(params: Vec<String>) -> Self {
        CreateCommand{ params }
    }

    fn is_valid(&self) -> bool {
        self.params.len() == 2
    }
}

impl Command for CreateCommand {
    fn execute(&self) {
        if self.is_valid() {
            match TaskManager::create_task(self.params[0].as_str(), self.params[1].as_str()) {
                Ok(()) => {}
                Err(err) => {
                    println!("Error creating the task: {}", err)
                }
            }
        } else {
            self.usage();
        }
    }

    fn usage(&self) {
        println!("\nCommand to create a {{UUID}}.task with a given title and description\n\
                  Usage:\n  cargo run create [title] [description]")
    }
}