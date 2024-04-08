use crate::command_trait::Command;
use crate::task_manager::TaskManager;
use crate::utils;

pub struct RemoveCommand {
    params: Vec<String>,
}

impl RemoveCommand {
    pub fn new(params: Vec<String>) -> Self {
        RemoveCommand{ params }
    }

    fn is_valid(&self) -> bool {
        self.params.len() == 1
    }

    fn remove_all_completed_tasks(&self) {
        match TaskManager::remove_all_completed_tasks() {
            Ok(()) => {}
            Err(err) => {
                println!("Error removing all the completed tasks: {}", err);
            }
        }
    }
    
    fn remove_task(&self, task_name: &str) {
        match TaskManager::remove_task(task_name) {
            Ok(()) => {}
            Err(err) => {
                println!("Error removing the task '{}': {}", task_name, err);
            }
        }
    }
}

impl Command for RemoveCommand {
    fn execute(&self) {
        if self.is_valid() {
            match self.params[0].as_str() {
                "completed" => {
                    return self.remove_all_completed_tasks();
                }
                _ => { 
                    if utils::is_valid_task_name(self.params[0].as_str()) {
                        return self.remove_task(self.params[0].as_str());
                    }
                }
            }
        } 
        self.usage();      
    }

    fn usage(&self) {
        println!("\nCommand to remove the task by it's name or to remove all the completed tasks\n\
                  Usage:\n  cargo run remove [ task_name | completed]")
    }
}