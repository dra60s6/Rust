use crate::command_trait::Command;
use crate::task_manager::TaskManager;
use crate::utils;

pub struct CompleteCommand {
    params: Vec<String>,
}

impl CompleteCommand {
    pub fn new(params: Vec<String>) -> Self {
        CompleteCommand{ params }
    }

    fn is_valid(&self) -> bool {
        self.params.len() == 1
    }
    
    fn mark_task_as_completed(&self) {
        match TaskManager::mark_task_as_completed(self.params[0].as_str()) {
            Ok(()) => {}
            Err(err) => {
                println!("Error completing the task: {}", err);
            }
        }
    }
}

impl Command for CompleteCommand {
    fn execute(&self) {
        if self.is_valid() && utils::is_valid_task_name(self.params[0].as_str()) {
            self.mark_task_as_completed();
        } else {
            self.usage();
        }
    }

    fn usage(&self) {
        println!("\nCommand to mark the task as completed by changing the task extension from {{UUID}}.task to {{UUID}}.done.\n\
                    Usage:\n  cargo run complete [ task_name ]")
    }
}
