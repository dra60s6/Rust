use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use crate::utils;
pub struct TaskManager;

impl TaskManager {

    pub fn create_task(title: &str, description: &str) -> io::Result<()> {
        if let Some(tasks_path) = utils::get_tasks_directory() {
            let task_name = utils::create_task_name();
            let task_path = tasks_path.join(task_name);
            let mut task = File::create(task_path)?;
            writeln!(task, "Title: {}\nDescription: {}", title, description)?;
        } else {
            println!("Failed to find tasks directory!");
        }
        Ok(())
    }

    pub fn remove_task(task_name: &str)-> io::Result<()> {
        if let Some(tasks_path)= utils::get_tasks_directory() {
            let task_path = tasks_path.join(task_name);
            if fs::metadata(task_path.as_os_str()).is_ok() {        
                fs::remove_file(task_path.as_os_str())?;
                println!("Successfully removed task '{}'", task_name);
            } else {
                println!("Task '{}' does not exist.", task_name);
            }
        } else {
            println!("Failed to find tasks directory!");
        }
        Ok(())
    }

    pub fn remove_all_completed_tasks() -> io::Result<()> {
        if let Some(tasks_path) = utils::get_tasks_directory() {
            for entry in fs::read_dir(tasks_path)? {
                let entry = entry?;
                let task_path = entry.path();

                if task_path.extension().unwrap() == utils::COMPLETED_TASK_EXTENSION {
                    fs::remove_file(task_path)?;
                }
            }
        } else {
            println!("Failed to find tasks directory!");
        }
        Ok(())
    }

    pub fn mark_task_as_completed(task_name: &str) -> io::Result<()> {
        if let Some(tasks_path) = utils::get_tasks_directory() {
            let task_path = tasks_path.join(task_name);
            if fs::metadata(task_path.as_os_str()).is_ok() { 
                let task_path = Path::new(task_path.as_os_str());

                if task_path.extension().unwrap() != utils::COMPLETED_TASK_EXTENSION {
                    let new_task_path = task_path.with_extension(utils::COMPLETED_TASK_EXTENSION);
                    fs::rename(task_path, new_task_path)?;
                    println!("Successfully marked task '{}' as completed.", task_name);
                } else {
                    println!("Task '{}' already completed.", task_name);
                }
            } else {
                println!("Task '{}' does not exist.", task_name);
            }
        } else {
            println!("Failed to find tasks directory!");
        }
        Ok(())
    }       

    pub fn view_task(task_name: &str) -> io::Result<()> {
        if let Some(tasks_path) = utils::get_tasks_directory() {
            let task_path = tasks_path.join(task_name);
            if fs::metadata(task_path.as_os_str()).is_ok() {
                let mut task = File::open(task_path)?;
                let mut content = String::new();
                task.read_to_string(&mut content)?;
                println!("Content of '{}':\n{}", task_name, content);
            } else {
                println!("Task '{}' does not exist.", task_name);
            }
        } else {
            println!("Failed to find tasks directory!");
        }
        Ok(())
    }

    pub fn view_all_tasks() -> io::Result<()> {
        if let Some(tasks_path) = utils::get_tasks_directory() {
            for entry in fs::read_dir(tasks_path)? {
                let entry = entry?;
                let task_name = entry.file_name().into_string().unwrap();
                println!("{}", task_name)
            }
        } else {
            println!("Failed to find tasks directory!");
        }
        Ok(())
    }    
}