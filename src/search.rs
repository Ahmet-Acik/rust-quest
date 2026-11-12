use crate::models::{Priority, Task};

/// Mission 4: Lifetimes (`'a`), Iterators, and Zero-Copy Borrowing
///
/// Notice the lifetime `'a`:
/// The returned `Vec<&'a Task>` holds references that point directly into the input `tasks` slice.
/// No `Task` or `String` is cloned!
pub fn search_by_keyword<'a>(tasks: &'a [Task], query: &str) -> Vec<&'a Task> {
    let query_lower = query.to_lowercase();
    tasks
        .iter()
        .filter(|task| {
            task.title.to_lowercase().contains(&query_lower)
                || task.tags.iter().any(|tag| tag.contains(&query_lower))
        })
        .collect()
}

/// Filter tasks by minimum priority using iterators and closures.
pub fn filter_by_min_priority<'a>(tasks: &'a [Task], min_priority: Priority) -> Vec<&'a Task> {
    tasks
        .iter()
        .filter(|t| t.priority.weight() >= min_priority.weight())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_borrows_without_cloning() {
        let mut t1 = Task::new(1, String::from("Master the borrow checker"), Priority::High);
        t1.add_tag("ownership");
        let t2 = Task::new(2, String::from("Buy groceries"), Priority::Low);
        let tasks = vec![t1, t2];

        let results = search_by_keyword(&tasks, "borrow");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 1);

        let tag_results = search_by_keyword(&tasks, "ownership");
        assert_eq!(tag_results.len(), 1);
    }
}

