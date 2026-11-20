//! Entry point for the `rust-quest` CLI binary.
//!
//! ☕ JAVA SIMILARITY:
//! - `fn main()` is the program entry point, just like `public static void main(String[] args)`.
//!
//! 🦀 RUST DIFFERENCE:
//! - In Rust, free functions can exist at the module level without being wrapped in a `class Main`.
//! - Notice the `!` in `println!(...)` and `vec![...]`: the `!` means it is a **compile-time macro**,
//!   not a normal function. That's how Rust checks your format string placeholders (`{}`) at
//!   **compile time** instead of throwing `IllegalFormatException` at runtime like `String.format()`!

use std::env;
use std::path::Path;

use rust_quest::concurrency::compute_stats_parallel;
use rust_quest::exercises::{parse_quick_task, sort_by_priority_desc};
use rust_quest::java_patterns::create_exporter;
use rust_quest::models::{Priority, Task};
use rust_quest::search::{filter_by_min_priority, search_by_keyword};
use rust_quest::storage::{load_tasks, save_tasks};

// `const` in Rust is a compile-time constant (like `private static final String DB_FILE = "tasks.tsv";` in Java).
const DB_FILE: &str = "tasks.tsv";

fn print_help() {
    // `r#"..."#` is a raw multi-line string literal (similar to Java 15+ text blocks `"""..."""`).
    println!(
        r#"🦀 RustQuest — CLI Task & Knowledge Manager (Java -> Rust Edition)
Usage:
  cargo run -- add "<title>" [low|medium|high] [tag1,tag2]
  cargo run -- quick "<title> | <priority> | [tag1,tag2]"
  cargo run -- list [--sort|-s]
  cargo run -- done <id>
  cargo run -- delete <id>
  cargo run -- search "<keyword>"
  cargo run -- filter <low|medium|high>
  cargo run -- export [markdown|csv]
  cargo run -- stats
  cargo run -- demo
"#
    );
}

fn seed_demo_tasks() -> Vec<Task> {
    // ☕ JAVA: Variables are mutable by default unless marked `final`.
    // 🦀 RUST: Variables (`let x`) are **immutable by default** (`final` by default!) unless marked `let mut x`.
    let mut t1 = Task::new(
        1,
        String::from("Understand Ownership & Borrowing (vs Java GC)"),
        Priority::High,
    );
    t1.add_tag("core");
    t1.add_tag("memory");

    let mut t2 = Task::new(
        2,
        String::from("Practice Enums & Pattern Matching (vs Java Sealed Types)"),
        Priority::High,
    );
    t2.add_tag("core");
    t2.add_tag("enums");
    t2.complete();

    let mut t3 = Task::new(
        3,
        String::from("Compare Traits, Result & Arc<Mutex<T>> with Core Java"),
        Priority::Medium,
    );
    t3.add_tag("practice");
    t3.add_tag("java-to-rust");

    // `vec![...]` macro is like Java's `new ArrayList<>(List.of(t1, t2, t3))`
    vec![t1, t2, t3]
}

fn main() {
    // ☕ JAVA: `String[] args` is passed directly into `main(String[] args)` (excluding program name).
    // 🦀 RUST: `env::args()` returns an iterator where `args[0]` is the binary path itself,
    // and `args[1]` is the first user argument.
    let args: Vec<String> = env::args().collect();
    let db_path = Path::new(DB_FILE);

    // ☕ JAVA: `try { tasks = loadTasks(dbPath); } catch (StorageException err) { ... }`
    // 🦀 RUST: Pattern match on `Result<Vec<Task>, StorageError>`!
    let mut tasks = match load_tasks(db_path) {
        Ok(loaded) => loaded,
        Err(err) => {
            eprintln!("Error loading tasks: {:?}", err);
            return;
        }
    };

    // ☕ JAVA: `String command = args.length > 1 ? args[1] : "help";`
    // 🦀 RUST: `args.get(1)` safely returns `Option<&String>` (never throws `IndexOutOfBoundsException`!).
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "demo" => {
            tasks = seed_demo_tasks();
            save_tasks(db_path, &tasks).expect("Failed to save demo tasks");
            println!("✨ Seeded 3 demo Rust learning tasks into '{}'!", DB_FILE);
            // Notice `&tasks`: we iterate over references so `tasks` isn't consumed/moved!
            for task in &tasks {
                println!("  {}", task);
            }
        }
        "add" => {
            let title = match args.get(2) {
                // Pattern guard `if !t.trim().is_empty()` right inside a `match` arm!
                Some(t) if !t.trim().is_empty() => t.clone(),
                _ => {
                    eprintln!("Usage: cargo run -- add \"<title>\" [low|medium|high] [tags]");
                    return;
                }
            };
            let priority_str = args.get(3).map(|s| s.as_str()).unwrap_or("medium");
            let priority = match Priority::from_str(priority_str) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };

            let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let mut task = Task::new(next_id, title, priority);

            if let Some(tags_raw) = args.get(4) {
                for tag in tags_raw.split(',') {
                    task.add_tag(tag);
                }
            }

            println!("✅ Added: {}", task);
            tasks.push(task);
            save_tasks(db_path, &tasks).expect("Failed to save tasks");
        }
        "quick" => {
            let raw = match args.get(2) {
                Some(r) if !r.trim().is_empty() => r,
                _ => {
                    eprintln!(
                        "Usage: cargo run -- quick \"<title> | <priority> | [tag1,tag2]\""
                    );
                    return;
                }
            };
            let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            match parse_quick_task(next_id, raw) {
                Ok(task) => {
                    println!("✅ Added: {}", task);
                    tasks.push(task);
                    save_tasks(db_path, &tasks).expect("Failed to save tasks");
                }
                Err(e) => eprintln!("Error parsing quick task: {}", e),
            }
        }
        "list" => {
            if tasks.is_empty() {
                println!("No tasks yet! Run `cargo run -- demo` to seed sample tasks.");
            } else {
                let sort_flag = matches!(args.get(2).map(|s| s.as_str()), Some("--sort" | "-s"));
                let display_tasks: Vec<Task>;
                let iter_slice: &[Task] = if sort_flag {
                    display_tasks = sort_by_priority_desc(&tasks);
                    &display_tasks
                } else {
                    &tasks
                };

                println!(
                    "📋 Your Tasks ({}{}):",
                    tasks.len(),
                    if sort_flag { ", sorted by priority" } else { "" }
                );
                for task in iter_slice {
                    println!("  {}", task);
                }
            }
        }
        "done" => {
            // ☕ JAVA: `Optional.ofNullable(...).flatMap(...)`
            // 🦀 RUST: `.and_then(...)` is `flatMap` on `Option`!
            let id: usize = match args.get(2).and_then(|s| s.parse().ok()) {
                Some(id) => id,
                None => {
                    eprintln!("Please provide a valid task ID: `cargo run -- done <id>`");
                    return;
                }
            };

            match tasks.iter_mut().find(|t| t.id == id) {
                Some(task) => {
                    task.complete();
                    println!("🎉 Completed: {}", task);
                    save_tasks(db_path, &tasks).expect("Failed to save tasks");
                }
                None => eprintln!("Task #{} not found.", id),
            }
        }
        "delete" => {
            let id: usize = match args.get(2).and_then(|s| s.parse().ok()) {
                Some(id) => id,
                None => {
                    eprintln!("Please provide a valid task ID: `cargo run -- delete <id>`");
                    return;
                }
            };

            let before_len = tasks.len();
            // ☕ JAVA: `tasks.removeIf(t -> t.getId() == id);`
            // 🦀 RUST: `tasks.retain(|t| t.id != id);` keeps elements where the predicate is `true`.
            tasks.retain(|t| t.id != id);

            if tasks.len() < before_len {
                save_tasks(db_path, &tasks).expect("Failed to save tasks");
                println!("🗑️  Deleted task #{}.", id);
            } else {
                eprintln!("Task #{} not found.", id);
            }
        }
        "search" => {
            let query = match args.get(2) {
                Some(q) => q,
                None => {
                    eprintln!("Usage: cargo run -- search \"<keyword>\"");
                    return;
                }
            };
            let matches = search_by_keyword(&tasks, query);
            println!("🔍 Found {} matching task(s) for '{}':", matches.len(), query);
            for task in matches {
                println!("  {}", task);
            }
        }
        "filter" => {
            let min_priority = match args.get(2) {
                Some(p) => match Priority::from_str(p) {
                    Ok(prio) => prio,
                    Err(e) => {
                        eprintln!("{}", e);
                        return;
                    }
                },
                None => {
                    eprintln!("Usage: cargo run -- filter <low|medium|high>");
                    return;
                }
            };
            let matches = filter_by_min_priority(&tasks, min_priority);
            println!(
                "🎯 Found {} task(s) with priority >= {}:",
                matches.len(),
                min_priority
            );
            for task in matches {
                println!("  {}", task);
            }
        }
        "stats" => {
            let stats = compute_stats_parallel(&tasks, 4);
            println!("📊 Task Statistics (computed across 4 worker threads):");
            println!("  Total:                 {}", stats.total);
            println!("  Completed:             {}", stats.completed);
            println!("  Pending:               {}", stats.pending);
            println!("  High-Priority Pending: {}", stats.high_priority_pending);
            if !stats.tag_counts.is_empty() {
                println!("  Tag Frequencies:");
                for (tag, count) in &stats.tag_counts {
                    println!("    #{:<12} -> {}", tag, count);
                }
            }
        }
        _ => print_help(),
    }
}
