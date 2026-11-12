use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use crate::models::Task;

/// Mission 5: Fearless Concurrency
/// Demonstrates spawning OS threads and communicating via channels (`std::sync::mpsc`).
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TaskStats {
    pub total: usize,
    pub completed: usize,
    pub pending: usize,
    pub high_priority_pending: usize,
    pub tag_counts: HashMap<String, usize>,
}

/// Splits tasks into chunks and processes tag frequencies across worker threads.
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

    // `thread::scope` (introduced in Rust 1.63) lets threads safely borrow local slices
    // without needing `Arc` or `'static` lifetimes!
    thread::scope(|s| {
        for chunk in tasks.chunks(chunk_size) {
            let tx_clone = tx.clone();
            s.spawn(move || {
                let mut local_counts: HashMap<String, usize> = HashMap::new();
                for task in chunk {
                    for tag in &task.tags {
                        *local_counts.entry(tag.clone()).or_insert(0) += 1;
                    }
                }
                let _ = tx_clone.send(local_counts);
            });
        }
        drop(tx); // Close original sender so `rx` finishes when all workers complete

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

