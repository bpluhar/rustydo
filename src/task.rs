use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub details: String,
    pub creation_date: DateTime<Utc>,
    pub completed_date: Option<DateTime<Utc>>,
    pub completed: bool
}

impl Task {
    pub fn new(title: String, details: String) -> io::Result<Self> {
        let task = Task {
            title,
            details,
            creation_date: Utc::now(),
            completed_date: None,
            completed: false,
        };
        
        task.save_to_file()?;
        Ok(task)
    }

    fn save_to_file(&self) -> io::Result<()> {
        const TASKS_FILE: &str = "tasks.json";
        
        // Read existing tasks
        let mut tasks = Self::read_all_tasks()?;
        tasks.push(self.clone());

        // Write all tasks back to file
        let json = serde_json::to_string_pretty(&tasks)?;
        fs::write(TASKS_FILE, json)?;
        
        Ok(())
    }

    pub fn read_all_tasks() -> io::Result<Vec<Task>> {
        const TASKS_FILE: &str = "tasks.json";
        
        // If file doesn't exist, create it with empty array
        if !Path::new(TASKS_FILE).exists() {
            fs::write(TASKS_FILE, "[]")?;
        }

        // Read and parse file
        let mut file = File::open(TASKS_FILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let tasks: Vec<Task> = serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            
        Ok(tasks)
    }

    pub fn complete(&mut self) -> io::Result<()> {
        self.completed = true;
        self.completed_date = Some(Utc::now());
        
        // Update the task in the file
        let mut tasks = Self::read_all_tasks()?;
        if let Some(task) = tasks.iter_mut().find(|t| t.title == self.title) {
            *task = self.clone();
            let json = serde_json::to_string_pretty(&tasks)?;
            fs::write("tasks.json", json)?;
        }
        
        Ok(())
    }
}