use std::fs;
use std::io;
use std::path::Path;

use crate::models::{Priority, Status, Task};

/// Mission 3: Error Handling (`Result`, `?` operator, and Custom Errors)
#[derive(Debug)]
pub enum StorageError {
    Io(io::Error),
    ParseError { line: usize, message: String },
}

impl From<io::Error> for StorageError {
    fn from(err: io::Error) -> Self {
        StorageError::Io(err)
    }
}

/// Saves a slice of tasks (`&[Task]`) to a plain-text TSV file.
/// Why `&[Task]` instead of `&Vec<Task>`?
/// Because a slice (`&[T]`) works with Vecs, arrays, and sub-slices!
pub fn save_tasks(path: &Path, tasks: &[Task]) -> Result<(), StorageError> {
    let mut content = String::new();
    for task in tasks {
        let status = match task.status {
            Status::Pending => "pending",
            Status::Completed => "completed",
        };
        let priority = match task.priority {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
        };
        let tags = task.tags.join(",");
        content.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            task.id, status, priority, tags, task.title
        ));
    }
    // The `?` operator automatically converts `io::Error` into `StorageError` via `From`!
    fs::write(path, content)?;
    Ok(())
}

/// Loads tasks from disk. If the file doesn't exist yet, returns an empty `Vec`.
pub fn load_tasks(path: &Path) -> Result<Vec<Task>, StorageError> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let raw = fs::read_to_string(path)?;
    let mut tasks = Vec::new();

    for (idx, line) in raw.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(5, '\t').collect();
        if parts.len() < 5 {
            return Err(StorageError::ParseError {
                line: idx + 1,
                message: String::from("Expected 5 tab-separated fields"),
            });
        }

        let id = parts[0].parse::<usize>().map_err(|_| StorageError::ParseError {
            line: idx + 1,
            message: format!("Invalid task ID: '{}'", parts[0]),
        })?;

        let status = match parts[1] {
            "completed" => Status::Completed,
            _ => Status::Pending,
        };

        let priority = Priority::from_str(parts[2]).map_err(|msg| StorageError::ParseError {
            line: idx + 1,
            message: msg,
        })?;

        let tags: Vec<String> = if parts[3].trim().is_empty() {
            Vec::new()
        } else {
            parts[3].split(',').map(|s| s.to_string()).collect()
        };

        let title = parts[4].to_string();

        tasks.push(Task {
            id,
            title,
            priority,
            status,
            tags,
        });
    }

    Ok(tasks)
}

