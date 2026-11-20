//! 🎯 HANDS-ON RUST EXERCISES (WITH JAVA COMPARISONS)
//!
//! Run `cargo test` in your terminal to test all solutions!

use crate::models::{Priority, Task};

/// Exercise 1: Ownership & Borrowing (`&Task` vs Java Object Reference)
/// Return a summary string for a task without taking ownership of `task`.
/// Format: `"#<id>: <title> (<PRIORITY>)"` e.g. `"#1: Learn Rust (HIGH)"`
///
/// ☕ JAVA SIMILARITY:
/// - `format!("#{}: {} ({})", ...)` works just like Java's `String.format("#%d: %s (%s)", ...)`.
///
/// 🦀 RUST DIFFERENCE:
/// - If this function took `task: Task` (by value) instead of `task: &Task` (by reference),
///   ownership of `task` would move into this function and `task` would be **destroyed** when
///   the function returns! Borrowing with `&Task` lets the caller keep using `task` afterwards.
pub fn format_task_summary(task: &Task) -> String {
    format!("#{}: {} ({})", task.id, task.title, task.priority)
}

/// Exercise 2: `Option<T>` & Pattern Matching (Why Rust Has No `null` or `NullPointerException`)
/// Find a mutable reference to a task by its `id` so the caller can modify it.
///
/// ☕ JAVA SIMILARITY:
/// - Directly comparable to Java's `Optional<Task>`:
///   `tasks.stream().filter(t -> t.getId() == id).findFirst()`
///
/// 🦀 RUST DIFFERENCE (`Option<&mut Task>` vs `null` / `Optional<Task>`):
/// - Rust has **no `null` keyword**. Every reference (`&T`, `&mut T`) is guaranteed at compile time
///   to point to a valid, initialized value.
/// - Whenever a value might be absent, Rust uses `Option<T>`, an enum with `Some(T)` or `None`.
/// - Unlike Java's `Optional<T>` (which is a heap-allocated wrapper box that can itself be `null`!),
///   `Option<&mut Task>` has the **Null-Pointer Optimization (NPO)**: at the machine-code level,
///   `Option<&mut Task>` occupies the exact same 8 bytes as a raw pointer (`0` represents `None`),
///   so it has **zero memory or runtime overhead**!
pub fn find_task_mut(tasks: &mut [Task], id: usize) -> Option<&mut Task> {
    tasks.iter_mut().find(|t| t.id == id)
}

/// Exercise 3: Iterators, Cloning & Custom Sorting (`Ord` vs Java `Comparator`)
/// Return a new `Vec<Task>` sorted from highest priority to lowest priority.
/// If two tasks have the same priority, sort by `id` ascending.
///
/// ☕ JAVA SIMILARITY:
/// - Chaining `.cmp(...).then_with(|| ...)` is the Rust equivalent of Java's `Comparator` chaining:
///   ```java
///   tasks.stream()
///        .sorted(Comparator.comparingInt((Task t) -> t.getPriority().weight()).reversed()
///                          .thenComparingInt(Task::getId))
///        .toList();
///   ```
///
/// 🦀 RUST DIFFERENCE (Explicit `.to_vec()` Clone vs Shallow Copy):
/// - In Java, copying a `List<Task>` into a new `ArrayList<>(tasks)` only copies object *references*
///   (mutating a `Task` in the sorted list would still mutate the original list's `Task`!).
/// - In Rust, `tasks.to_vec()` performs a deep `.clone()` of each `Task` so the returned `Vec<Task>`
///   is completely independent of the original slice.
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

/// Exercise 4: `Result<T, E>` & The `?` Operator (Errors as Values vs Java Exceptions)
/// Parse a quick-add string formatted as `"title | priority | tag1,tag2"`
/// Example: `"Build a CLI | high | rust,cli"`
/// If the title is empty or priority is invalid, return `Err(String)`.
///
/// ☕ JAVA SIMILARITY:
/// - Splitting and trimming (`raw.split('|').map(|s| s.trim()).collect()`) mirrors Java's
///   `Arrays.stream(raw.split("\\|")).map(String::trim).toList()`.
///
/// 🦀 RUST DIFFERENCE:
/// - Notice `parts.get(2)` returns `Option<&&str>` instead of throwing `ArrayIndexOutOfBoundsException`!
///   Using `if let Some(tags_part) = parts.get(2)` safely handles optional 3rd segments.
/// - `let priority = Priority::from_str(parts[1])?;` unwraps `Ok(Priority)` or short-circuits and
///   returns `Err(String)` immediately—replacing `try / catch` blocks with a single `?` character.
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
        // `task` is still valid here because `format_task_summary` borrowed it (`&task`)!
        assert_eq!(task.id, 1);
    }

    #[test]
    fn exercise_2_find_mut() {
        let mut tasks = vec![
            Task::new(1, String::from("Task A"), Priority::Low),
            Task::new(2, String::from("Task B"), Priority::High),
        ];

        // `if let Some(t) = ...` is Rust's ergonomic replacement for `if (opt.isPresent()) { var t = opt.get(); ... }`
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
