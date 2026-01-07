use chrono::{DateTime, Local, NaiveDate, TimeZone};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub is_complete: bool,
    pub scheduled: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: 0,
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
                "[ID:{}] - [{}] {} (Scheduled: {})\n",
                self.id,
                check_mark,
                self.title,
                dt.format("%Y-%m-%d")
            ),
            None => format!("[ID:{}] - [{}] {}\n", self.id, check_mark, self.title,),
        }
    }

    pub fn from_md_line(line: &str) -> Option<Self> {
        // Regex Breakdown:
        // ^\[ID:(\d+)\]      -> Starts with [ID:digits], capture digits (Group 1)
        // \s-\s              -> " - " separator
        // \[(.)\]            -> [x] or [ ], capture the character (Group 2)
        // \s                 -> space
        // (.*?)              -> Capture the Title lazy (Group 3)
        // (?:\s\(Scheduled:\s(.*)\))?$ -> Optional Non-capturing group for schedule. Capture date (Group 4)

        let re = Regex::new(r"^\[ID:(\d+)\] - \[(.)\] (.*?)(?: \(Scheduled: (.*)\))?$").unwrap();

        let caps = re.captures(line)?;

        let id = caps.get(1)?.as_str().parse::<u32>().ok()?;

        let is_complete = caps.get(2)?.as_str() == "x";

        let title = caps.get(3)?.as_str().to_string();

        let scheduled = match caps.get(4) {
            Some(m) => {
                let date_str = m.as_str();
                NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .ok()
                    .and_then(|d| d.and_hms_opt(0, 0, 0))
                    .and_then(|dt| Local.from_local_datetime(&dt).single())
            }
            None => None,
        };

        Some(Task {
            id,
            title,
            is_complete,
            scheduled,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskRepository {
    pub tasks: HashMap<u32, Task>,
    pub next_id: u32,
}

impl TaskRepository {
    pub fn new() -> Self {
        TaskRepository {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: String, scheduled: Option<DateTime<Local>>) {
        let id = self.next_id;

        let task = Task {
            id,
            title,
            is_complete: false,
            scheduled,
        };

        self.tasks.insert(id, task);
        self.next_id += 1;
    }

    pub fn get(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }

    pub fn remove(&mut self, id: u32) -> Option<Task> {
        self.tasks.remove(&id)
    }

    pub fn list_all(&self) -> Vec<&Task> {
        let mut list: Vec<&Task> = self.tasks.values().collect();
        list.sort_by_key(|t| t.id);
        list
    }

    pub fn load_from_file(
        file_path: &PathBuf,
    ) -> Result<TaskRepository, Box<dyn std::error::Error>> {
        let mut repo = TaskRepository::new();

        if !file_path.exists() {
            return Ok(repo);
        }

        let file = std::fs::File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line: String = line?;
            if line.trim().is_empty() {
                continue;
            }

            if let Some(task) = Task::from_md_line(&line) {
                if task.id >= repo.next_id {
                    repo.next_id = task.id + 1;
                }
                repo.tasks.insert(task.id, task);
            }
        }

        Ok(repo)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // Overwrite file
            .open(path)?;

        for task in self.list_all() {
            writeln!(file, "{}", task.to_md_line())?;
        }

        Ok(())
    }
}
