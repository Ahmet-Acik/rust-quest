use std::fs;
use std::io;
use std::path::Path;

use crate::models::{Priority, Status, Task};

// ============================================================================
// MISSION 3: Error Handling & File I/O (`Result` & `?` vs Java Exceptions)
// ============================================================================

/// Custom error type for storage operations.
///
/// ☕ JAVA SIMILARITY:
/// - Serves the same domain purpose as a custom checked exception hierarchy in Java:
///   `public class StorageException extends Exception { ... }`.
///
/// 🦀 RUST DIFFERENCE (Errors as Values, Not Exceptions):
/// - Rust has **no `try / catch / throws`** and no exception stack unwinding for recoverable errors!
/// - Instead, fallible functions return `Result<T, E>` (an enum with `Ok(T)` or `Err(E)`).
/// - Like Java checked exceptions, the compiler forces you to acknowledge `Result` (you cannot
///   silently ignore it without a compiler warning), but without the boilerplate of `try/catch`
///   thanks to the `?` operator shown below.
/// - For unrecoverable bugs (like Java's `AssertionError` or `IndexOutOfBoundsException`), Rust
///   uses `panic!("message")`.
#[derive(Debug)]
pub enum StorageError {
    Io(io::Error),
    ParseError { line: usize, message: String },
}

/// Converts a standard `std::io::Error` into our custom `StorageError`.
///
/// ☕ JAVA SIMILARITY:
/// - Like a constructor `public StorageException(IOException cause) { super(cause); }`
///   used when wrapping low-level exceptions (`catch (IOException e) { throw new StorageException(e); }`).
///
/// 🦀 RUST DIFFERENCE:
/// - Implementing the standard `From<io::Error>` trait teaches the `?` operator how to convert
///   `io::Error` into `StorageError` **automatically**—so you never have to write `try / catch / rethrow`!
impl From<io::Error> for StorageError {
    fn from(err: io::Error) -> Self {
        StorageError::Io(err)
    }
}

/// Saves a slice of tasks (`&[Task]`) to a plain-text TSV file.
///
/// ☕ JAVA SIMILARITY:
/// - Similar to `public static void saveTasks(Path path, List<Task> tasks) throws StorageException`.
/// - `fs::write(path, content)` is equivalent to Java's `Files.writeString(path, content)`.
/// - Returning `Result<(), StorageError>` uses `()` (the empty tuple / "unit" type), which is
///   Rust's equivalent of Java's `void` (or `Void` in generics like `Callable<Void>`).
///
/// 🦀 RUST DIFFERENCE (`&[Task]` Slice vs `&Vec<Task>` and RAII vs `try-with-resources`):
/// - Why accept `tasks: &[Task]` (a borrowed slice) instead of `&Vec<Task>`?
///   A slice `&[T]` is a lightweight 2-word view `(pointer, length)` into *any* contiguous sequence
///   (a `Vec<Task>`, a fixed-size array `[Task; 3]`, or a sub-range `&tasks[1..3]`).
/// - **No `try-with-resources` needed:** In Java, file handles must be closed with
///   `try (var writer = ...) { ... }` (`AutoCloseable`) because the GC doesn't run immediately.
///   In Rust, resources use **RAII (`Drop` trait)**: the underlying OS file descriptor created inside
///   `fs::write` is closed automatically the exact instant it goes out of scope!
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
    // ☕ JAVA EQUIVALENT:
    //   try { Files.writeString(path, content); }
    //   catch (IOException e) { throw new StorageException(e); }
    //
    // 🦀 RUST `?` OPERATOR:
    //   If `fs::write` returns `Err(io_err)`, `?` immediately converts `io_err` into `StorageError`
    //   (via `From<io::Error>`) and returns `Err(...)` from `save_tasks`. If `Ok(())`, execution continues!
    fs::write(path, content)?;
    Ok(())
}

/// Loads tasks from disk. If the file doesn't exist yet, returns an empty `Vec`.
///
/// ☕ JAVA SIMILARITY:
/// - Similar to `public static List<Task> loadTasks(Path path) throws StorageException`.
///
/// 🦀 RUST DIFFERENCE:
/// - `.map_err(|_| ...)?` transforms one error type (e.g. `ParseIntError`, like Java's
///   `NumberFormatException`) into our domain `StorageError::ParseError` inline in a functional chain,
///   instead of wrapping a block in `try { Integer.parseInt(...) } catch (NumberFormatException e) { ... }`.
pub fn load_tasks(path: &Path) -> Result<Vec<Task>, StorageError> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let raw = fs::read_to_string(path)?;
    let mut tasks = Vec::new();

    // `.lines().enumerate()` gives `(index, line_slice)` pairs without allocating a `String` per line!
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

        // ☕ JAVA: `int id = Integer.parseInt(parts[0]);` (throws unchecked NumberFormatException)
        // 🦀 RUST: `.parse::<usize>()` returns a `Result` (using the "turbofish" `::<usize>` syntax
        // to tell the compiler which type to parse into), which we map to `StorageError` and unwrap with `?`.
        let id = parts[0]
            .parse::<usize>()
            .map_err(|_| StorageError::ParseError {
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

        // Notice field shorthand syntax: `{ id, title, priority, status, tags }`
        // instead of `id: id, title: title, ...` when variable names match struct field names.
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
