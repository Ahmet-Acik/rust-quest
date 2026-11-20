use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::models::Task;

// ============================================================================
// MISSION 5: Fearless Concurrency (Java Threads, `synchronized` & Channels)
// ============================================================================

/// Aggregated statistics across all tasks.
///
/// ☕ JAVA SIMILARITY:
/// - `HashMap<String, usize>` is Rust's equivalent of `java.util.HashMap<String, Integer>`.
/// - `#[derive(Default)]` generates a zero-initialized default constructor (`TaskStats::default()`),
///   just like Java's default zero/empty field initialization.
///
/// 🦀 RUST DIFFERENCE:
/// - No boxing! In Java, `HashMap<String, Integer>` boxes every primitive `int` into a heap
///   `Integer` object. In Rust, `HashMap<String, usize>` stores raw `usize` integers directly in the table.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TaskStats {
    pub total: usize,
    pub completed: usize,
    pub pending: usize,
    pub high_priority_pending: usize,
    pub tag_counts: HashMap<String, usize>,
}

/// Pattern 1: Message-Passing Concurrency (`thread::scope` + `mpsc::channel`).
///
/// ☕ JAVA SIMILARITY:
/// - Spawning worker threads and sending partial results over `mpsc::channel()` (Multi-Producer,
///   Single-Consumer queue) is similar to Java worker threads pushing into a `LinkedBlockingQueue<Map<String, Integer>>`
///   or using Java 21's `StructuredTaskScope`.
///
/// 🦀 RUST DIFFERENCE (Compile-Time Data Race Prevention via `Send` & `Sync`):
/// - In Java, nothing stops two threads from accidentally reading and mutating the same `ArrayList`
///   or `HashMap` without synchronization, leading to runtime data corruption or `ConcurrentModificationException`.
/// - In Rust, the compiler checks two automatic marker traits on every thread boundary:
///   - `Send`: Safe to transfer ownership of a value to another thread.
///   - `Sync`: Safe to share an immutable reference (`&T`) across multiple threads simultaneously.
/// - Because `chunk` is an immutable slice (`&[Task]`), multiple threads can safely read it in parallel
///   with zero locks and zero defensive copying!
/// - `move || { ... }` on the closure transfers ownership of `tx_clone` into the spawned thread,
///   And once `tx_clone.send(local_counts)` is called, the worker thread **loses ownership** of
///   `local_counts`—so it is impossible for the worker thread to mutate the map after sending it!
pub fn compute_stats_parallel(tasks: &[Task], worker_count: usize) -> TaskStats {
    let total = tasks.len();
    let completed = tasks.iter().filter(|t| t.is_completed()).count();
    let pending = total - completed;
    let high_priority_pending = tasks
        .iter()
        .filter(|t| !t.is_completed() && t.priority.weight() == 3)
        .count();

    if tasks.is_empty() {
        return TaskStats::default();
    }

    let workers = worker_count.max(1);
    let chunk_size = (tasks.len() + workers - 1) / workers;
    let (tx, rx) = mpsc::channel();

    // `thread::scope` guarantees all spawned threads join before the scope block returns,
    // allowing worker threads to safely borrow the local `tasks` slice without `Arc`!
    thread::scope(|s| {
        for chunk in tasks.chunks(chunk_size) {
            let tx_clone = tx.clone();
            s.spawn(move || {
                let mut local_counts: HashMap<String, usize> = HashMap::new();
                for task in chunk {
                    for tag in &task.tags {
                        // ☕ JAVA: `localCounts.merge(tag, 1, Integer::sum);`
                        // 🦀 RUST: `.entry(...).or_insert(0)` returns a mutable reference (`&mut usize`)
                        // directly into the map slot, which we dereference with `*` to increment in-place.
                        *local_counts.entry(tag.clone()).or_insert(0) += 1;
                    }
                }
                let _ = tx_clone.send(local_counts);
            });
        }
        drop(tx); // Close original sender so `rx` finishes iterating when all workers complete

        let mut merged_tags = HashMap::new();
        for partial in rx {
            for (tag, count) in partial {
                *merged_tags.entry(tag).or_insert(0) += count;
            }
        }

        TaskStats {
            total,
            completed,
            pending,
            high_priority_pending,
            tag_counts: merged_tags,
        }
    })
}

/// Pattern 2: Shared-State Concurrency (`Arc<Mutex<T>>` vs Java `synchronized`).
///
/// ☕ JAVA SIMILARITY:
/// - In Java, multiple threads can share a reference to a `Map<String, Integer>` and guard access
///   using `synchronized (sharedMap) { ... }` or `ReentrantLock`.
///
/// 🦀 RUST DIFFERENCE (`Arc<Mutex<T>>` — Why the Lock Wraps the Data):
/// - **1. `Arc<T>` (Atomic Reference Counting) replaces Garbage-Collected shared pointers:**
///   Because Rust has no GC, if multiple threads need shared ownership of heap data, you wrap it in
///   `Arc::new(...)`. Calling `Arc::clone(&shared)` does NOT clone the `HashMap`—it just increments
///   an atomic reference counter (like copying a Java object reference!).
/// - **2. `Mutex<T>` wraps the data *inside* the lock:**
///   In Java, the lock and the data are separate in the type system—a developer can forget to write
///   `synchronized (map)` and accidentally mutate `map` unsafely.
///   In Rust, the `HashMap` lives *inside* `Mutex<HashMap<String, usize>>`. You literally **cannot**
///   touch the `HashMap` without calling `.lock().unwrap()`, and the lock is **automatically unlocked**
///   via RAII (`Drop`) the moment the guard variable `map` goes out of scope (no `finally { lock.unlock(); }`!).
pub fn compute_tag_counts_with_mutex(
    tasks: &[Task],
    worker_count: usize,
) -> HashMap<String, usize> {
    if tasks.is_empty() {
        return HashMap::new();
    }

    let shared_counts: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let workers = worker_count.max(1);
    let chunk_size = (tasks.len() + workers - 1) / workers;

    thread::scope(|s| {
        for chunk in tasks.chunks(chunk_size) {
            // Increment atomic reference count (cheap pointer copy, like passing a Java reference)
            let shared_ref = Arc::clone(&shared_counts);
            s.spawn(move || {
                for task in chunk {
                    for tag in &task.tags {
                        // Acquire mutex lock. When `guard` goes out of scope at `}`, the lock releases automatically!
                        let mut guard = shared_ref.lock().expect("Mutex poisoned");
                        *guard.entry(tag.clone()).or_insert(0) += 1;
                    }
                }
            });
        }
    });

    // Extract the final HashMap once all worker threads have finished
    Arc::try_unwrap(shared_counts)
        .expect("All worker threads have joined")
        .into_inner()
        .expect("Mutex not poisoned")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Priority;

    #[test]
    fn test_message_passing_and_shared_mutex_match() {
        let mut t1 = Task::new(1, String::from("Task 1"), Priority::High);
        t1.add_tag("rust");
        t1.add_tag("concurrency");

        let mut t2 = Task::new(2, String::from("Task 2"), Priority::Medium);
        t2.add_tag("rust");
        t2.complete();

        let tasks = vec![t1, t2];

        let stats = compute_stats_parallel(&tasks, 2);
        let mutex_counts = compute_tag_counts_with_mutex(&tasks, 2);

        assert_eq!(stats.total, 2);
        assert_eq!(stats.completed, 1);
        assert_eq!(stats.pending, 1);
        assert_eq!(stats.high_priority_pending, 1);
        assert_eq!(stats.tag_counts.get("rust"), Some(&2));
        assert_eq!(stats.tag_counts.get("concurrency"), Some(&1));
        assert_eq!(stats.tag_counts, mutex_counts);
    }
}
