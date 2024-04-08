use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use regex::Regex;

const TASKS_DIRECTORY: &str = "Tasks";  //
const TASK_EXTENSION: &str = "task";
pub const COMPLETED_TASK_EXTENSION: &str = "done";

pub fn is_valid_task_name(task_name: &str) -> bool {
    // task name format: {UUID}.task or {UUID}.done
    let reg_ex = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\.(task|done)$").unwrap();
    reg_ex.is_match(task_name)
}

pub fn create_task_name() -> String {
    format!("{}.{}", Uuid::new_v4(), TASK_EXTENSION)
}

pub fn get_tasks_directory() -> Option<PathBuf> {
    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => {
            println!("Failed to get home directory!");
            return None;
        }
    };
    let tasks_path = home_dir.join(TASKS_DIRECTORY);
    Some(tasks_path)
}

// All the tasks will be created and managed in/from HOME_DIR/Tasks path
pub fn create_tasks_directory() -> Option<PathBuf> {
    if let Some(tasks_path) = get_tasks_directory() {
        if !tasks_path.exists() {
            match fs::create_dir(&tasks_path) {
                Ok(()) => return Some(tasks_path),
                Err(err) => {
                    println!("Error to create directory path {} : {}", tasks_path.display(), err);
                    return None;
                },
            }
        }
        return Some(tasks_path);
    }
    return None;
}