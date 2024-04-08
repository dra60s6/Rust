use crate::command_trait::Command;
use crate::task_manager::TaskManager;
use crate::utils;

pub struct ViewCommand {
    params: Vec<String>,
}

impl ViewCommand {
    pub fn new(params: Vec<String>) -> Self {
        ViewCommand{ params }
    }

    fn is_valid(&self) -> bool {
        self.params.len() == 1
    }
    
    fn view_task(&self, task_name: &str) {
        match TaskManager::view_task(task_name) {
            Ok(()) => {}
            Err(err) => {
                println!("Error viewing the task '{}': {}", task_name, err);
            }
        }
    }

    fn view_all_tasks(&self) {
        match TaskManager::view_all_tasks() {
            Ok(()) => {}
            Err(err) => {
                println!("Error viewing all tasks: {}", err);
            }
        }
    }
}

impl Command for ViewCommand {
    fn execute(&self) {
        if self.is_valid() {
            match self.params[0].as_str() {
                "all" => {
                    return self.view_all_tasks();
                }
                _ => {
                    if utils::is_valid_task_name(self.params[0].as_str()) {
                        return self.view_task(self.params[0].as_str());
                    }
                }
            }
        }
        self.usage();
    }

    fn usage(&self) {
        println!("\nCommand to view task's details or all the created tasks.\n\
                 Usage:\n  cargo run view [ task_name | all ]")
    }
}