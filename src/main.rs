mod task_manager;
mod command_trait;
mod command_types;
mod utils;

use crate::command_trait::Command;
use crate::command_types::{ create_command::CreateCommand, 
                            remove_command::RemoveCommand, 
                            complete_command::CompleteCommand, 
                            view_command::ViewCommand };
use std::env;

fn app_usage() {
    println!("\nUsage: \n  cargo run [ COMMAND ] [ PARAMETERS ]\n\
            Commands:\n  \
             create [ title ] [ description ]    Create a {{UUID}}.task with a title and a description as content\n  \
             remove [ task_name | completed ]    Remove the task or all the completed tasks\n  \
             complete [ task_name ]              Mark the task as completed (changing task's extension from .task to .done)\n  \
             view [ task_name | all ]            View the task details or all the available tasks\n");
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        app_usage();
        return;
    }

    utils::create_tasks_directory();
    let command = &args[1];
    let parameters: Vec<String> = args.iter().skip(2).cloned().collect();
    
    let command: Box<dyn Command> = match command.as_str() {
        "create" => Box::new(CreateCommand::new(parameters)),
        "remove" => Box::new(RemoveCommand::new(parameters)),
        "complete" => Box::new(CompleteCommand::new(parameters)),
        "view" => Box::new(ViewCommand::new(parameters)),     
        _ => {
            println!("Invalid command!");
            app_usage();
            return;
        }
    };
    command.execute();
}