use std::fmt;

/// Mission 1: Enums & Pattern Matching
/// In Rust, enums are algebraic data types that can represent distinct variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    /// Parse a string slice (`&str`) into a `Priority` using `Result`.
    pub fn from_str(input: &str) -> Result<Self, String> {
        match input.trim().to_lowercase().as_str() {
            "low" | "l" => Ok(Priority::Low),
            "medium" | "med" | "m" => Ok(Priority::Medium),
            "high" | "h" => Ok(Priority::High),
            other => Err(format!("Invalid priority '{}'. Use low, medium, or high.", other)),
        }
    }

    /// 🎯 EXERCISE 1 (Easy):
    /// Return a numeric weight for sorting tasks by priority:
    /// Low -> 1, Medium -> 2, High -> 3
    pub fn weight(&self) -> u8 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
        }
    }
}

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
/// Notice how `Completed` can hold data (a note or timestamp) if you want to extend it!
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Pending,
    Completed,
}

/// Mission 2: Structs, Ownership & Borrowing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    pub tags: Vec<String>,
}

impl Task {
    /// Constructor pattern in Rust (`Task::new(...)`).
    /// Takes ownership of `title` (`String`) so the struct owns its heap data.
    pub fn new(id: usize, title: String, priority: Priority) -> Self {
        Self {
            id,
            title,
            priority,
            status: Status::Pending,
            tags: Vec::new(),
        }
    }

    /// Borrows `self` mutably (`&mut self`) to update state in-place without copying.
    pub fn complete(&mut self) {
        self.status = Status::Completed;
    }

    /// Borrows `self` immutably (`&self`) to inspect state.
    pub fn is_completed(&self) -> bool {
        matches!(self.status, Status::Completed)
    }

    /// 🎯 PRACTICE CHALLENGE A:
    /// Add a tag to `self.tags` only if it isn't already present (case-insensitive).
    /// Notice how `&mut self` lets us mutate the vector, and `&str` avoids unnecessary allocations
    /// if the tag already exists!
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

#[cfg(test)]
mod tests {
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
}

