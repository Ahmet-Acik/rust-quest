# RustQuest (`rust-quest`)

Welcome to **RustQuest**, a hands-on learning project designed to teach you core Rust concepts by reading, running, and extending a real CLI Task & Knowledge Manager with zero external dependencies.

---

## Quick Start

Open `/Users/drahmetacik/Projects/rust-quest` as your active workspace, then run:

```bash
# 1. Seed sample tasks and view them
cargo run -- demo
cargo run -- list

# 2. Add a new task with priority and tags (or use pipe-separated quick add)
cargo run -- add "Finish Rust exercises" high rust,learning
cargo run -- quick "Build a CLI | medium | rust,cli"

# 3. List tasks (optionally sorted by priority descending)
cargo run -- list --sort

# 4. Search & filter tasks (zero-copy borrowing with lifetimes)
cargo run -- search "rust"
cargo run -- filter high

# 5. Complete or delete a task
cargo run -- done 1
cargo run -- delete 2

# 6. Compute stats across worker threads
cargo run -- stats

# 7. Run the built-in unit tests
cargo test
```

---

## Your 5-Mission Learning Path

Read the source files in this order:

1. **Mission 1 & 2 — Enums, Structs & Borrowing (`src/models.rs`)**
   - See how `Priority` and `Status` use algebraic data types (`enum`) and pattern matching (`match`).
   - Compare owned data (`String`, `Vec<String>`) vs borrowed slices (`&str`, `&self`, `&mut self`).
   - See how `fmt::Display` is implemented (Rust's trait system).

2. **Mission 3 — Error Handling & File I/O (`src/storage.rs`)**
   - Learn how Rust replaces exceptions with `Result<T, E>` and the `?` operator.
   - See how `From<io::Error>` lets `?` automatically convert error types.

3. **Mission 4 — Lifetimes & Iterators (`src/search.rs`)**
   - Understand explicit lifetimes (`fn search_by_keyword<'a>(tasks: &'a [Task], ...) -> Vec<&'a Task>`) to filter data without cloning strings.

4. **Mission 5 — Fearless Concurrency (`src/concurrency.rs`)**
   - See how `std::thread::scope` and `std::sync::mpsc::channel` split work across OS threads safely without data races.

5. ** Your Coding Challenges (`src/exercises.rs`)**
   - Open `src/exercises.rs` where 4 hands-on functions have `todo!()` placeholders.
   - Implement each function and verify your solution by running:
     ```bash
     cargo test -- --ignored
     ```

---

## Next Level Extensions (Once you finish `exercises.rs`)
- **Milestone A:** Add a `delete <id>` command in `src/main.rs` using `Vec::retain`.
- **Milestone B:** Replace the TSV storage in `src/storage.rs` with JSON using `serde` and `serde_json` (`cargo add serde --features derive && cargo add serde_json`).
- **Milestone C:** Replace manual `env::args()` parsing in `src/main.rs` with `clap` (`cargo add clap --features derive`).

