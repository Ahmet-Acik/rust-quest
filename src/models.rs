use std::fmt;

// ============================================================================
// MISSION 1: Enums, Pattern Matching & `#[derive]` (Java Enums / Sealed Types)
// ============================================================================

/// Task priority level.
///
/// ☕ JAVA SIMILARITY:
/// - Like `public enum Priority { LOW, MEDIUM, HIGH }` in Java, this defines a fixed set
///   of named constants and can have methods attached via an `impl` block.
/// - `#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]` auto-generates boilerplate at
///   compile time—very similar to Java `record` methods or Lombok's `@Data` / `@EqualsAndHashCode`,
///   plus implementing Java's `Cloneable` and `Comparable<Priority>`.
///
/// 🦀 RUST DIFFERENCE:
/// - `Copy` means values are copied by simple bitwise stack copy (like Java primitive `int` or `boolean`),
///   with zero heap allocation or garbage collector pointer tracking!
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    /// Parse a string slice (`&str`) into a `Priority` using `Result`.
    ///
    /// ☕ JAVA SIMILARITY:
    /// - Similar to a static factory method in Java: `public static Priority fromStr(String input)`.
    ///
    /// 🦀 RUST DIFFERENCE:
    /// - Notice there is no `self` parameter: in Rust, functions inside `impl` without `self`
    ///   are **associated functions** (Java `static` methods), called with `Priority::from_str(...)`.
    /// - Input is `&str` (a borrowed view into UTF-8 bytes, zero allocation) instead of a heap `String`.
    /// - Instead of `throws IllegalArgumentException`, it returns `Result<Self, String>` where
    ///   `Ok(value)` is success and `Err(error)` is failure.
    pub fn from_str(input: &str) -> Result<Self, String> {
        // ☕ JAVA SIMILARITY: Like Java 14+ `switch` expression (`return switch (s) { case "low", "l" -> ... }`).
        // 🦀 RUST DIFFERENCE: `match` in Rust is exhaustive at compile time and supports patterns,
        // guards, and destructuring.
        match input.trim().to_lowercase().as_str() {
            "low" | "l" => Ok(Priority::Low),
            "medium" | "med" | "m" => Ok(Priority::Medium),
            "high" | "h" => Ok(Priority::High),
            other => Err(format!(
                "Invalid priority '{}'. Use low, medium, or high.",
                other
            )),
        }
    }

    /// Return a numeric weight for sorting tasks by priority:
    /// Low -> 1, Medium -> 2, High -> 3
    ///
    /// ☕ JAVA SIMILARITY: Like an instance method `public byte weight()` using an implicit `this`.
    /// 🦀 RUST DIFFERENCE: Rust makes `this` explicit via `&self` (an immutable borrow of the instance).
    /// Also notice there is no `return` keyword or trailing semicolon on `match`—in Rust, almost
    /// everything (`match`, `if/else`, `{ ... }` blocks) is an **expression** whose last value is returned!
    pub fn weight(&self) -> u8 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
        }
    }
}

/// ☕ JAVA SIMILARITY:
/// - Implementing `fmt::Display` is the Rust equivalent of overriding `public String toString()`
///   in `java.lang.Object`.
///
/// 🦀 RUST DIFFERENCE:
/// - There is no root `Object` class in Rust! Instead, formatting is opt-in via two traits:
///   1. `#[derive(Debug)]` (`{:?}`) -> for developer logs/debugging.
///   2. `fmt::Display` (`{}`)      -> for user-facing output.
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Priority::Low => "LOW",
            Priority::Medium => "MED",
            Priority::High => "HIGH",
        };
        write!(f, "{}", label)
    }
}

/// Task completion status.
///
/// ☕ JAVA SIMILARITY:
/// - In simple cases (`Pending`, `Completed`), this looks like a regular Java `enum`.
///
/// 🦀 RUST DIFFERENCE (Algebraic Data Types / Tagged Unions):
/// - In Java, every enum constant has the exact same fields.
/// - In Rust, each enum variant can hold **completely different data**—e.g. you could define
///   `Completed { completed_at: u64, note: String }` while `Pending` holds nothing!
/// - In modern Java (Java 17+), you would need a `sealed interface Status permits Pending, Completed`
///   plus two separate `record` classes to achieve what a single Rust `enum` does natively.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Pending,
    Completed,
}

// ============================================================================
// MISSION 2: Structs, Ownership & Borrowing (Java Classes, GC & References)
// ============================================================================

/// Represents a task in our manager.
///
/// ☕ JAVA SIMILARITY:
/// - A `struct` holds named fields just like a Java `class` or `record`.
/// - `Vec<String>` is Rust's growable array, equivalent to Java's `ArrayList<String>`.
///
/// 🦀 RUST DIFFERENCE:
/// - **No Class Inheritance:** Rust structs cannot `extend` another struct. Polymorphism is done
///   strictly through **traits** (interfaces) and composition.
/// - **No `null`:** Every field below MUST hold a valid value. If a field were optional, its type
///   would explicitly be `Option<String>` (similar to Java's `Optional<String>`, but enforced by the compiler).
/// - **Contiguous Memory Layout (`Vec<T>`):** In Java, `ArrayList<Task>` is an array of heap pointers
///   to separately allocated `Task` objects. In Rust, `Vec<Task>` stores the `Task` structs inline
///   next to each other in one contiguous block of memory (cache-friendly, fewer allocations).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: usize, // `usize` is pointer-sized unsigned int (64-bit on 64-bit CPUs; Java uses 32-bit signed `int` for indices)
    pub title: String, // Owned heap-allocated UTF-8 string (growable like Java `StringBuilder`, owned by `Task`)
    pub priority: Priority,
    pub status: Status,
    pub tags: Vec<String>,
}

impl Task {
    /// Constructor pattern in Rust (`Task::new(...)`).
    ///
    /// ☕ JAVA SIMILARITY:
    /// - Acts like `public Task(int id, String title, Priority priority)` in Java.
    ///
    /// 🦀 RUST DIFFERENCE (Ownership & Move Semantics):
    /// - Rust has no `new` keyword! By convention, types provide a `pub fn new(...) -> Self` function.
    /// - Passing `title: String` by value **moves ownership** of that heap buffer into the `Task` struct.
    ///   In Java, passing a `String` copies a reference and the Garbage Collector tracks both;
    ///   in Rust, there is no GC: `Task` becomes the single owner of `title`, and when `Task` goes
    ///   out of scope, `title` and `tags` are freed immediately and deterministically.
    pub fn new(id: usize, title: String, priority: Priority) -> Self {
        Self {
            id,
            title,
            priority,
            status: Status::Pending,
            tags: Vec::new(),
        }
    }

    /// Marks the task as completed.
    ///
    /// ☕ JAVA SIMILARITY:
    /// - Like a Java setter/mutator method `public void complete() { this.status = Status.COMPLETED; }`.
    ///
    /// 🦀 RUST DIFFERENCE (Explicit Mutability & The Borrow Checker):
    /// - In Java, any method on a non-final field can mutate `this` at any time.
    /// - In Rust, mutation requires `&mut self` (an **exclusive mutable borrow**).
    /// - **Golden Rule of Rust Borrowing:** At any given time, you can have EITHER:
    ///   - Any number of immutable references (`&T` — multiple readers), OR
    ///   - Exactly ONE mutable reference (`&mut T` — single writer, no other readers/writers).
    ///   This prevents Java's `ConcurrentModificationException` and iterator invalidation at compile time!
    pub fn complete(&mut self) {
        self.status = Status::Completed;
    }

    /// Checks whether the task is completed.
    ///
    /// ☕ JAVA SIMILARITY:
    /// - Like a Java getter `public boolean isCompleted()`.
    ///
    /// 🦀 RUST DIFFERENCE:
    /// - Takes `&self` (shared immutable borrow): guaranteed by the compiler not to mutate the `Task`.
    pub fn is_completed(&self) -> bool {
        matches!(self.status, Status::Completed)
    }

    /// Adds a tag to `self.tags` only if it isn't already present (case-insensitive).
    ///
    /// ☕ JAVA SIMILARITY:
    /// - `.iter().any(|t| t == &normalized)` is just like Java's
    ///   `this.tags.stream().anyMatch(t -> t.equals(normalized))`.
    ///
    /// 🦀 RUST DIFFERENCE (`&str` vs `String` and `==` vs `.equals()`):
    /// - Notice the parameter is `tag: &str` (a borrowed string view) so callers can pass either
    ///   a string literal `"rust"` or a borrowed `&my_string` without allocating a new `String` upfront.
    /// - In Java, `t == normalized` compares object reference addresses (a classic Java bug!), so you
    ///   must call `t.equals(normalized)`. In Rust, `==` invokes the `PartialEq` trait—comparing
    ///   actual string contents by value!
    pub fn add_tag(&mut self, tag: &str) {
        let normalized = tag.trim().to_lowercase();
        if !normalized.is_empty() && !self.tags.iter().any(|t| t == &normalized) {
            self.tags.push(normalized);
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let check = if self.is_completed() { "x" } else { " " };
        let tags_str = if self.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", self.tags.join(", "))
        };
        write!(
            f,
            "[{}] #{:<3} ({:<4}) {}{}",
            check, self.id, self.priority, self.title, tags_str
        )
    }
}

// ============================================================================
// BONUS FOR JAVA DEVS: Traits & Generics vs Java Interfaces
// ============================================================================

/// A trait defining shared behavior across types.
///
/// ☕ JAVA SIMILARITY:
/// - A `trait` is Rust's `interface`.
/// - Just like Java 8+ interfaces support `default` methods, Rust traits can provide default
///   implementations (`headline_badge`) that call required methods (`summary`).
///
/// 🦀 RUST DIFFERENCE:
/// - In Java, a class must declare `class Task implements Summarizable` when the class is defined.
/// - In Rust, `impl Summarizable for Task` is a separate block. You can even implement your own
///   trait on an existing standard-library type like `String` or `i32` (extension traits without wrapper classes)!
pub trait Summarizable {
    /// Required method (like an abstract interface method in Java).
    fn summary(&self) -> String;

    /// Default method (like a `default String headlineBadge()` method in a Java interface).
    fn headline_badge(&self) -> String {
        format!("[TaskSummary] {}", self.summary())
    }
}

impl Summarizable for Task {
    fn summary(&self) -> String {
        format!("#{}: {} ({})", self.id, self.title, self.priority)
    }
}

/// Generic function constrained by a trait bound (`T: Summarizable`).
///
/// ☕ JAVA SIMILARITY:
/// - Looks like Java generics: `public static <T extends Summarizable> String formatBadge(T item)`.
///
/// 🦀 RUST DIFFERENCE (Monomorphization vs Type Erasure):
/// - Java uses **type erasure** (generics are erased to `Object`/`Summarizable` at runtime and
///   dispatched via virtual method tables).
/// - Rust uses **monomorphization**: the compiler generates a specialized, inlineable version of
///   `format_badge` for each concrete type `T` used, resulting in zero runtime virtual-dispatch cost!
///   (If you actually want runtime polymorphism like a Java interface reference, Rust uses `dyn Summarizable`).
pub fn format_badge<T: Summarizable>(item: &T) -> String {
    item.headline_badge()
}

#[cfg(test)]
mod tests {
    // ☕ JAVA SIMILARITY: Like JUnit `@Test` methods!
    // 🦀 RUST DIFFERENCE: Unit tests live right next to the code they test inside a `#[cfg(test)]`
    // module (which is completely stripped out of production builds) and can test private functions directly.
    use super::*;

    #[test]
    fn test_priority_parsing() {
        assert_eq!(Priority::from_str("high"), Ok(Priority::High));
        assert_eq!(Priority::from_str("M"), Ok(Priority::Medium));
        assert!(Priority::from_str("urgent").is_err());
    }

    #[test]
    fn test_task_lifecycle_and_tags() {
        let mut task = Task::new(1, String::from("Learn Rust ownership"), Priority::High);
        assert!(!task.is_completed());

        task.add_tag("Rust");
        task.add_tag("rust"); // Duplicate (case-insensitive) should be ignored
        task.add_tag("systems");

        assert_eq!(task.tags, vec!["rust", "systems"]);

        task.complete();
        assert!(task.is_completed());
    }

    #[test]
    fn test_trait_and_generics() {
        let task = Task::new(7, String::from("Compare Traits vs Interfaces"), Priority::Medium);
        assert_eq!(
            format_badge(&task),
            "[TaskSummary] #7: Compare Traits vs Interfaces (MED)"
        );
    }
}
