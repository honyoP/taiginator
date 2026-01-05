use chrono::{DateTime, Local};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub struct Task {
    pub title: String,
    pub is_complete: bool,
    pub scheduled: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            title,
            is_complete: false,
            scheduled: None,
        }
    }

    pub fn scheduled(mut self, date: Option<DateTime<Local>>) -> Self {
        self.scheduled = date;
        self
    }

    pub fn to_md_line(&self) -> String {
        let check_mark = if self.is_complete { "x" } else { " " };
        match &self.scheduled {
            Some(dt) => format!(
                "- [{}] {} (Scheduled: {})\n",
                check_mark,
                self.title,
                dt.format("%Y-%m-%d")
            ),
            None => format!("- [{}] {}\n", check_mark, self.title,),
        }
    }
}

pub fn load_tasks(file_path: &PathBuf) -> io::Result<String> {
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

pub fn add_task(
    file_path: &PathBuf,
    task_title: String,
    task_schedule: Option<DateTime<Local>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_task = Task::new(task_title).scheduled(task_schedule);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    file.write_all(new_task.to_md_line().as_bytes())?;

    println!("Task added successfully!");
    Ok(())
}
