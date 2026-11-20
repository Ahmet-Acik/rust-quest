//! # Mission 6: Core Java Patterns & Mental Models in Rust
//!
//! This module demonstrates the 6 essential Core Java concepts that work differently in Rust:
//! 1. **Runtime Polymorphism & Strategy Pattern:** `Box<dyn Trait>` / `&dyn Trait` vs. Java Interface References
//! 2. **No Constructor/Method Overloading:** Builder Pattern + `Into<String>` & `From` conversion traits
//! 3. **Custom Resource Cleanup (RAII):** Implementing `Drop` vs. Java `AutoCloseable` (`try-with-resources`) & `finally`
//! 4. **Closures & Functional Interfaces:** `Fn`, `FnMut`, `FnOnce` vs. Java `Predicate<T>`, `Consumer<T>`, `Function<T, R>`
//! 5. **Collections Framework:** `Hash` + `Eq`, `BTreeMap` (`Ord` vs. `TreeMap`), and Polymorphic `.collect()`
//! 6. **Shared Ownership & Interior Mutability:** `Rc<RefCell<T>>` vs. Java Single-Threaded Object Graphs

use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;

use crate::models::{Priority, Status, Task};

// ============================================================================
// 1. RUNTIME POLYMORPHISM: `Box<dyn Trait>` vs. Java Interface References
// ============================================================================

/// Strategy interface for exporting tasks in different formats.
///
/// ☕ JAVA SIMILARITY:
/// - Just like `public interface TaskExporter { String export(List<Task> tasks); }`.
///
/// 🦀 RUST DIFFERENCE (Static Dispatch `<T: Trait>` vs. Dynamic Dispatch `dyn Trait`):
/// - In `src/models.rs`, we saw generic functions (`fn format_badge<T: Summarizable>(item: &T)`),
///   which the compiler duplicates for each concrete type at compile time (*static dispatch*).
/// - What if you choose the exporter at **runtime** (e.g. from a CLI flag `"markdown"` vs `"csv"`),
///   or want a heterogeneous `Vec` containing both `MarkdownExporter` and `CsvExporter`?
/// - In Java, `List<TaskExporter>` works automatically because every object is a heap pointer.
/// - In Rust, you use a **Trait Object**: `Box<dyn TaskExporter>` (owned on heap) or `&dyn TaskExporter`
///   (borrowed). Under the hood, a trait object is a 2-word **fat pointer**:
///   `(pointer_to_struct_data, pointer_to_vtable_of_methods)`.
pub trait TaskExporter {
    fn format_name(&self) -> &'static str;
    fn export(&self, tasks: &[Task]) -> String;
}

pub struct MarkdownExporter;

impl TaskExporter for MarkdownExporter {
    fn format_name(&self) -> &'static str {
        "Markdown"
    }

    fn export(&self, tasks: &[Task]) -> String {
        let mut out = String::from("# Tasks\n");
        for t in tasks {
            let box_char = if t.is_completed() { "x" } else { " " };
            out.push_str(&format!(
                "- [{}] **#{}** ({}) {}\n",
                box_char, t.id, t.priority, t.title
            ));
        }
        out
    }
}

pub struct CsvExporter;

impl TaskExporter for CsvExporter {
    fn format_name(&self) -> &'static str {
        "CSV"
    }

    fn export(&self, tasks: &[Task]) -> String {
        let mut out = String::from("id,priority,status,title\n");
        for t in tasks {
            let status = if t.is_completed() { "completed" } else { "pending" };
            out.push_str(&format!("{},{},{},{}\n", t.id, t.priority, status, t.title));
        }
        out
    }
}

/// Factory function returning a heap-allocated Trait Object (`Box<dyn TaskExporter>`).
///
/// ☕ JAVA SIMILARITY:
/// ```java
/// public static TaskExporter createExporter(String format) {
///     return switch (format) {
///         case "csv" -> new CsvExporter();
///         default -> new MarkdownExporter();
///     };
/// }
/// ```
///
/// 🦀 RUST DIFFERENCE:
/// - `MarkdownExporter` and `CsvExporter` are distinct concrete types! Returning `Box<dyn TaskExporter>`
///   allocates the chosen struct on the heap (`Box::new(...)`) and erases its concrete type behind the
///   `dyn TaskExporter` vtable—giving you Java-style virtual method dispatch when you need it.
pub fn create_exporter(format: &str) -> Box<dyn TaskExporter> {
    match format.trim().to_lowercase().as_str() {
        "csv" => Box::new(CsvExporter),
        _ => Box::new(MarkdownExporter),
    }
}

// ============================================================================
// 2. NO OVERLOADING: Builder Pattern & `Into<String>` Conversion Traits
// ============================================================================

/// Fluent builder for `Task`.
///
/// ☕ JAVA SIMILARITY:
/// - Looks just like a Java / Lombok `@Builder`:
///   `TaskBuilder.new(1, "Write Rust").priority(Priority.HIGH).tag("core").build()`.
///
/// 🦀 RUST DIFFERENCE (Why Builders & `impl Into<String>` Replace Overloading):
/// - Java classes often have 3–4 overloaded constructors (`Task(id, title)`, `Task(id, title, priority)`).
///   Rust has **no constructor or method overloading** and **no default parameter values**.
/// - Notice `title: impl Into<String>` on `TaskBuilder::new`:
///   In Java, if you want a method to accept either a `String` or a `CharSequence`, you either
///   overload it or convert manually. In Rust, `impl Into<String>` accepts *anything* that can
///   convert into a `String`—both a string literal `"Learn Rust"` (`&str`) and an owned `String`!
/// - Also notice `pub fn priority(mut self, p: Priority) -> Self`: it takes `self` **by value**
///   (moving ownership through each chained call) instead of mutating a heap pointer in place.
pub struct TaskBuilder {
    id: usize,
    title: String,
    priority: Priority,
    status: Status,
    tags: Vec<String>,
}

impl TaskBuilder {
    pub fn new(id: usize, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            priority: Priority::Medium, // Sensible default!
            status: Status::Pending,
            tags: Vec::new(),
        }
    }

    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn completed(mut self) -> Self {
        self.status = Status::Completed;
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        let normalized = tag.trim().to_lowercase();
        if !normalized.is_empty() && !self.tags.contains(&normalized) {
            self.tags.push(normalized);
        }
        self
    }

    pub fn build(self) -> Task {
        Task {
            id: self.id,
            title: self.title,
            priority: self.priority,
            status: self.status,
            tags: self.tags,
        }
    }
}

// ============================================================================
// 3. DETERMINISTIC CLEANUP (RAII): `Drop` Trait vs. Java `AutoCloseable` & `finally`
// ============================================================================

/// A scoped audit guard that logs when an operation finishes—even on early return!
///
/// ☕ JAVA SIMILARITY:
/// - In Java, to guarantee cleanup code runs when a scope exits, you implement `AutoCloseable`
///   and use `try (var guard = new AuditScope(...)) { ... }` or a `try { ... } finally { ... }` block.
///
/// 🦀 RUST DIFFERENCE (`Drop` Trait vs. `AutoCloseable` & Garbage Collector `finalize()`):
/// - Java's old `Object.finalize()` was unpredictable and deprecated because the GC runs at an
///   unknown future time. Even with `AutoCloseable`, if a caller forgets `try (...)`, the resource leaks!
/// - In Rust, implementing `Drop` guarantees the compiler will automatically call `drop(&mut self)`
///   at the exact closing brace `}` where the variable goes out of scope (or when `?` returns early,
///   or even during a `panic!` unwind). Callers **cannot** forget to close/drop it!
pub struct AuditGuard<'a> {
    operation: &'a str,
    log: &'a mut Vec<String>,
}

impl<'a> AuditGuard<'a> {
    pub fn enter(operation: &'a str, log: &'a mut Vec<String>) -> Self {
        log.push(format!("START: {}", operation));
        Self { operation, log }
    }
}

impl<'a> Drop for AuditGuard<'a> {
    fn drop(&mut self) {
        // Automatically executed the exact instant `AuditGuard` leaves scope!
        self.log.push(format!("END: {}", self.operation));
    }
}

// ============================================================================
// 4. CLOSURES: `Fn`, `FnMut`, `FnOnce` vs. Java `Predicate`, `Consumer` & "Effectively Final"
// ============================================================================

/// Applies a mutable visitor closure to every task matching a predicate closure.
///
/// ☕ JAVA SIMILARITY:
/// - In Java, you use `java.util.function.Predicate<Task>` and `Consumer<Task>`:
///   `void visitMatching(List<Task> tasks, Predicate<Task> predicate, Consumer<Task> visitor)`
///
/// 🦀 RUST DIFFERENCE (`Fn` vs. `FnMut` vs. `FnOnce` — Goodbye "Effectively Final"!):
/// - Have you ever hit Java's compiler error:
///   *"Variable used in lambda expression should be final or effectively final"*
///   and had to wrap a simple `int count = 0` inside `AtomicInteger` or `int[] count = {0}`?
/// - Java forces that restriction because a Java lambda could be stored and run on another thread
///   after the stack frame returns.
/// - Rust's borrow checker knows the exact lifetime of a closure! So Rust has 3 closure traits:
///   1. `Fn(&Task) -> bool`: Borrows captured variables immutably (`&`). Can be called repeatedly.
///   2. `FnMut(&Task)`: Borrows captured variables **mutably** (`&mut`). Can directly increment
///      a local `let mut count = 0;` on the stack without any `AtomicInteger` wrapper!
///   3. `FnOnce()`: Takes ownership of captured variables (consumes them). Can be called once.
pub fn inspect_matching_tasks<P, V>(tasks: &[Task], predicate: P, mut visitor: V)
where
    P: Fn(&Task) -> bool,
    V: FnMut(&Task),
{
    for task in tasks {
        if predicate(task) {
            visitor(task);
        }
    }
}

// ============================================================================
// 5. COLLECTIONS FRAMEWORK: `BTreeMap`, `HashSet` & Polymorphic `.collect()`
// ============================================================================

/// Groups tasks by `Priority` in sorted order (`Low` -> `Medium` -> `High`) and extracts unique tags.
///
/// ☕ JAVA SIMILARITY:
/// - `BTreeMap<Priority, Vec<&Task>>` is Rust's `java.util.TreeMap<Priority, List<Task>>`
///   (keys are kept sorted using `Ord`, which is Java's `Comparable<T>`).
/// - `HashSet<String>` is Rust's `java.util.HashSet<String>` (requires `Eq + Hash`, which is
///   Java's `equals()` + `hashCode()` contract).
///
/// 🦀 RUST DIFFERENCE (Polymorphic `.collect()` & Transpose `Result<Vec<T>, E>`):
/// - In Java Streams, you must pass a collector object: `.collect(Collectors.toSet())` or
///   `.collect(Collectors.toList())`.
/// - In Rust, `.collect()` is powered by the `FromIterator` trait: the **return type** (or turbofish
///   `.collect::<HashSet<String>>()`) determines what collection is built!
/// - Even cooler: if you have an iterator of `Result<Task, String>`, you can `.collect()` it directly
///   into a `Result<Vec<Task>, String>`—if every item is `Ok`, you get `Ok(Vec<Task>)`; if any item
///   fails, `.collect()` short-circuits and returns the first `Err(String)`!
pub fn group_by_priority_and_unique_tags(
    tasks: &[Task],
) -> (BTreeMap<Priority, Vec<&Task>>, HashSet<String>) {
    let mut grouped: BTreeMap<Priority, Vec<&Task>> = BTreeMap::new();
    for task in tasks {
        grouped.entry(task.priority).or_default().push(task);
    }

    // Notice `.flat_map(...)` (Java `Stream.flatMap`) + `.collect::<HashSet<String>>()`!
    let unique_tags: HashSet<String> = tasks
        .iter()
        .flat_map(|t| t.tags.iter().cloned())
        .collect();

    (grouped, unique_tags)
}

/// Parses a batch of quick-task strings, failing fast on the first invalid line.
/// Demonstrates collecting an `Iterator<Item = Result<T, E>>` into `Result<Vec<T>, E>`!
pub fn parse_batch_fail_fast(lines: &[&str]) -> Result<Vec<Task>, String> {
    lines
        .iter()
        .enumerate()
        .map(|(idx, line)| crate::exercises::parse_quick_task(idx + 1, line))
        .collect() // <-- Magic! Collects `Iterator<Result<Task, String>>` into `Result<Vec<Task>, String>`
}

// ============================================================================
// 6. SHARED OWNERSHIP & INTERIOR MUTABILITY: `Rc<RefCell<T>>` vs. Java Object Graphs
// ============================================================================

/// A shared in-memory event sink that multiple components can hold a reference to and mutate
/// within a single thread (e.g., Observer / Listener pattern).
///
/// ☕ JAVA SIMILARITY:
/// - In Java, two service objects (`TaskAdder` and `TaskCompleter`) can both hold a field
///   `private final List<String> auditLog;` pointing to the exact same `ArrayList` and call
///   `auditLog.add(...)` whenever they want.
///
/// 🦀 RUST DIFFERENCE (`Rc<RefCell<T>>` — Runtime Borrow Checking):
/// - Why doesn't Rust allow two structs to hold `&mut Vec<String>` to the same vector at the same time?
///   Because of the **Single Mutable Alias Rule** (at most ONE `&mut T` at compile time).
/// - When you genuinely need a Java-style shared object graph inside a single thread:
///   1. `Rc<T>` (**Reference Counted** pointer): Allows multiple owners of the same heap data in a
///      single thread (cheaper than `Arc<T>` because it uses non-atomic counters).
///   2. `RefCell<T>` (**Interior Mutability**): Moves Rust's borrow checker from **compile time to runtime**.
///      You call `.borrow()` for shared read access or `.borrow_mut()` for exclusive write access.
#[derive(Clone)]
pub struct SharedAuditSink {
    events: Rc<RefCell<Vec<String>>>,
}

impl SharedAuditSink {
    pub fn new() -> Self {
        Self {
            events: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn record(&self, event: impl Into<String>) {
        // Notice `&self` is an IMMUTABLE borrow of `SharedAuditSink`, yet `.borrow_mut()`
        // lets us safely mutate the inner `Vec<String>` ("interior mutability")!
        self.events.borrow_mut().push(event.into());
    }

    pub fn snapshot(&self) -> Vec<String> {
        self.events.borrow().clone()
    }
}

impl Default for SharedAuditSink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_dynamic_dispatch_strategy_pattern() {
        let task = TaskBuilder::new(1, "Learn dyn Trait")
            .priority(Priority::High)
            .completed()
            .build();
        let tasks = vec![task];

        // Heterogeneous list of trait objects (just like `List<TaskExporter>` in Java!)
        let exporters: Vec<Box<dyn TaskExporter>> =
            vec![create_exporter("markdown"), create_exporter("csv")];

        assert_eq!(exporters[0].format_name(), "Markdown");
        assert!(exporters[0].export(&tasks).contains("- [x] **#1** (HIGH) Learn dyn Trait"));

        assert_eq!(exporters[1].format_name(), "CSV");
        assert!(exporters[1].export(&tasks).contains("1,HIGH,completed,Learn dyn Trait"));
    }

    #[test]
    fn test_2_builder_pattern_and_into_string() {
        // Works with both `&str` literal and owned `String` thanks to `impl Into<String>`!
        let t1 = TaskBuilder::new(1, "From &str literal")
            .priority(Priority::Low)
            .tag("Builder")
            .tag("builder") // duplicate ignored
            .build();

        assert_eq!(t1.title, "From &str literal");
        assert_eq!(t1.priority, Priority::Low);
        assert_eq!(t1.tags, vec!["builder"]);
    }

    #[test]
    fn test_3_raii_drop_runs_automatically_at_scope_exit() {
        let mut log = Vec::new();
        {
            let _guard = AuditGuard::enter("save_database", &mut log);
            // Notice we never call `_guard.close()`!
            // It drops automatically at the closing brace `}` below.
        }
        assert_eq!(log, vec!["START: save_database", "END: save_database"]);
    }

    #[test]
    fn test_4_fnmut_closure_mutates_local_variable_without_atomic_wrapper() {
        let tasks = vec![
            TaskBuilder::new(1, "High A").priority(Priority::High).build(),
            TaskBuilder::new(2, "Low B").priority(Priority::Low).build(),
            TaskBuilder::new(3, "High C").priority(Priority::High).build(),
        ];

        // In Java, a lambda cannot do `matchedCount++` on a local `int matchedCount = 0;`.
        // In Rust, `FnMut` allows the closure to borrow `matched_count` and `titles` mutably!
        let mut matched_count = 0;
        let mut titles = Vec::new();

        inspect_matching_tasks(
            &tasks,
            |t| t.priority == Priority::High,
            |t| {
                matched_count += 1;
                titles.push(t.title.clone());
            },
        );

        assert_eq!(matched_count, 2);
        assert_eq!(titles, vec!["High A", "High C"]);
    }

    #[test]
    fn test_5_btreemap_hashset_and_result_collect() {
        let tasks = vec![
            TaskBuilder::new(1, "High task").priority(Priority::High).tag("rust").build(),
            TaskBuilder::new(2, "Low task").priority(Priority::Low).tag("java").tag("rust").build(),
        ];

        let (grouped, unique_tags) = group_by_priority_and_unique_tags(&tasks);

        // BTreeMap keys iterate in sorted order: Low first, then High
        let ordered_priorities: Vec<Priority> = grouped.keys().copied().collect();
        assert_eq!(ordered_priorities, vec![Priority::Low, Priority::High]);
        assert_eq!(unique_tags.len(), 2);
        assert!(unique_tags.contains("rust") && unique_tags.contains("java"));

        // Collecting `Iterator<Result<Task, String>>` into `Result<Vec<Task>, String>`
        let ok_batch = parse_batch_fail_fast(&["Task 1 | low | tag1", "Task 2 | high | tag2"]);
        assert_eq!(ok_batch.unwrap().len(), 2);

        let err_batch = parse_batch_fail_fast(&["Task 1 | low", "Bad Task | invalid_priority"]);
        assert!(err_batch.is_err());
    }

    #[test]
    fn test_6_rc_refcell_shared_observer_sink() {
        let sink = SharedAuditSink::new();
        // Clone the `Rc` pointer (both `observer_a` and `observer_b` point to the SAME inner `Vec`)
        let observer_a = sink.clone();
        let observer_b = sink.clone();

        observer_a.record("Task #1 created");
        observer_b.record("Task #1 completed");

        assert_eq!(
            sink.snapshot(),
            vec!["Task #1 created", "Task #1 completed"]
        );
    }
}
