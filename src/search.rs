use crate::models::{Priority, Task};

// ============================================================================
// MISSION 4: Lifetimes (`'a`), Iterators & Closures (Java Streams & GC)
// ============================================================================

/// Searches tasks by keyword across titles and tags without cloning any data.
///
/// ☕ JAVA SIMILARITY:
/// - The iterator chain `.iter().filter(|task| ...).collect()` looks and reads almost identically
///   to Java's Stream API:
///   ```java
///   tasks.stream()
///        .filter(task -> task.getTitle().toLowerCase().contains(q)
///                     || task.getTags().stream().anyMatch(tag -> tag.contains(q)))
///        .collect(Collectors.toList());
///   ```
/// - Closure syntax `|task| { ... }` is Rust's lambda syntax (`task -> { ... }` in Java).
///
/// 🦀 RUST DIFFERENCE 1 (Explicit Lifetimes `'a` vs Java Garbage Collector):
/// - Look at the signature: `fn search_by_keyword<'a>(tasks: &'a [Task], query: &str) -> Vec<&'a Task>`
/// - Because the function takes *two* references (`tasks` and `query`) and returns a `Vec` of
///   references (`&'a Task`), the Rust compiler asks: *"Do the returned references point into
///   `tasks` or into `query`?"*
/// - By annotating both `tasks: &'a [Task]` and `Vec<&'a Task>` with the same lifetime `'a`,
///   we tell the compiler: *"The returned `Task` references borrow from `tasks` (not `query`),
///   so the caller cannot drop or mutate `tasks` while these search results are still in use."*
/// - In Java, the Garbage Collector keeps `Task` objects alive on the heap as long as any list
///   references them. In Rust, `'a` enforces memory safety at **compile time** with zero GC overhead!
///
/// 🦀 RUST DIFFERENCE 2 (Zero-Cost Iterators & Unrestricted Closure Capture):
/// - In Java, `stream().filter(...)` allocates pipeline stage objects on the heap, and lambdas can
///   only capture `final` or "effectively final" local variables.
/// - In Rust, iterators are **zero-cost abstractions**—the compiler inlines the entire `.iter().filter()`
///   chain into a single tight machine-code loop, and closures can borrow local variables mutably or immutably!
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
///
/// ☕ JAVA SIMILARITY:
/// - Equivalent to `tasks.stream().filter(t -> t.getPriority().weight() >= min.weight()).toList()`.
///
/// 🦀 RUST DIFFERENCE (Lifetime Elision):
/// - Here there is only ONE input reference (`tasks: &'a [Task]`), so you could actually omit `'a`
///   (`pub fn filter_by_min_priority(tasks: &[Task], min_priority: Priority) -> Vec<&Task>`)
///   and Rust's **lifetime elision rules** would infer `'a` automatically! We write `'a` explicitly
///   here so you can see how input and output borrows connect.
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

    #[test]
    fn test_filter_by_min_priority() {
        let tasks = vec![
            Task::new(1, String::from("Low task"), Priority::Low),
            Task::new(2, String::from("Med task"), Priority::Medium),
            Task::new(3, String::from("High task"), Priority::High),
        ];

        let med_and_up = filter_by_min_priority(&tasks, Priority::Medium);
        let ids: Vec<usize> = med_and_up.iter().map(|t| t.id).collect();
        assert_eq!(ids, vec![2, 3]);
    }
}
