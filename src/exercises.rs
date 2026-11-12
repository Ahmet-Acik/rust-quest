//! 🎯 HANDS-ON RUST EXERCISES
//!
//! Run `cargo test` in your terminal to test all solutions!

use crate::models::{Priority, Task};

/// Exercise 1: Ownership & Borrowing
/// Return a summary string for a task without taking ownership of `task`.
/// Format: `"#<id>: <title> (<PRIORITY>)"` e.g. `"#1: Learn Rust (HIGH)"`
pub fn format_task_summary(task: &Task) -> String {
    format!("#{}: {} ({})", task.id, task.title, task.priority)
}

/// Exercise 2: Option & Pattern Matching
/// Find a mutable reference to a task by its `id` so the caller can modify it.
/// Hint: Use `tasks.iter_mut().find(|t| ...)`
pub fn find_task_mut(tasks: &mut [Task], id: usize) -> Option<&mut Task> {
    tasks.iter_mut().find(|t| t.id == id)
}

/// Exercise 3: Iterators & Sorting
/// Return a new `Vec<Task>` sorted from highest priority to lowest priority.
/// If two tasks have the same priority, sort by `id` ascending.
pub fn sort_by_priority_desc(tasks: &[Task]) -> Vec<Task> {
    let mut sorted = tasks.to_vec();
    sorted.sort_by(|a, b| {
        b.priority
            .weight()
            .cmp(&a.priority.weight())
            .then_with(|| a.id.cmp(&b.id))
    });
    sorted
}

/// Exercise 4: Result & Error Handling
/// Parse a quick-add string formatted as `"title | priority | tag1,tag2"`
/// Example: `"Build a CLI | high | rust,cli"`
/// If the title is empty or priority is invalid, return `Err(String)`.
pub fn parse_quick_task(id: usize, raw: &str) -> Result<Task, String> {
    let parts: Vec<&str> = raw.split('|').map(|s| s.trim()).collect();
    if parts.len() < 2 {
        return Err(String::from(
            "Expected format: 'title | priority | optional_tags'",
        ));
    }

    let title = parts[0];
    if title.is_empty() {
        return Err(String::from("Task title cannot be empty"));
    }

    // The `?` operator unwraps `Ok(Priority)` or returns early with `Err(String)`
    let priority = Priority::from_str(parts[1])?;
    let mut task = Task::new(id, title.to_string(), priority);

    if let Some(tags_part) = parts.get(2) {
        for tag in tags_part.split(',') {
            task.add_tag(tag);
        }
    }

    Ok(task)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_1_summary() {
        let task = Task::new(1, String::from("Learn Rust"), Priority::High);
        assert_eq!(format_task_summary(&task), "#1: Learn Rust (HIGH)");
        // `task` is still valid here because `format_task_summary` borrowed it!
        assert_eq!(task.id, 1);
    }

    #[test]
    fn exercise_2_find_mut() {
        let mut tasks = vec![
            Task::new(1, String::from("Task A"), Priority::Low),
            Task::new(2, String::from("Task B"), Priority::High),
        ];

        if let Some(t) = find_task_mut(&mut tasks, 2) {
            t.complete();
        }
        assert!(tasks[1].is_completed());
        assert!(find_task_mut(&mut tasks, 99).is_none());
    }

    #[test]
    fn exercise_3_sort() {
        let tasks = vec![
            Task::new(1, String::from("Low item"), Priority::Low),
            Task::new(2, String::from("High item"), Priority::High),
            Task::new(3, String::from("Med item"), Priority::Medium),
        ];

        let sorted = sort_by_priority_desc(&tasks);
        let ids: Vec<usize> = sorted.iter().map(|t| t.id).collect();
        assert_eq!(ids, vec![2, 3, 1]);
    }

    #[test]
    fn exercise_4_quick_parse() {
        let task = parse_quick_task(10, "Build a CLI | high | rust,cli").unwrap();
        assert_eq!(task.id, 10);
        assert_eq!(task.title, "Build a CLI");
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.tags, vec!["rust", "cli"]);

        assert!(parse_quick_task(11, "   | high | rust").is_err());
        assert!(parse_quick_task(12, "Valid title | invalid_prio | rust").is_err());
    }
}
