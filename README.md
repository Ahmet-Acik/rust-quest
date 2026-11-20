# RustQuest (`rust-quest`) — Rust for Java Developers Edition

Welcome to **RustQuest**, a hands-on learning project designed specifically for **Java developers transitioning to Rust**. Every source file contains side-by-side inline comments (`☕ JAVA SIMILARITY` and `🦀 RUST DIFFERENCE`) showing how Rust maps to Core Java—and where Rust fundamentally differs.

---

## Core Java vs. Rust — Mental Model Cheat Sheet

| Concept | ☕ Core Java | 🦀 Rust Equivalent | Key Difference & Where to See It |
| :--- | :--- | :--- | :--- |
| **Packages & Visibility** | `package`, `import`, package-private default | `mod`, `use`, **private by default** (`pub` to expose) | [src/lib.rs](src/lib.rs) |
| **Build & Dependencies** | Maven (`pom.xml`) / Gradle (`build.gradle`) | Cargo ([Cargo.toml](Cargo.toml)) | `cargo build`, `cargo run`, `cargo test` |
| **Variables** | Mutable by default (`final` for immutable) | **Immutable by default** (`let x`; use `let mut x` to mutate) | [src/main.rs](src/main.rs) |
| **Classes & Records** | `class` / `record` with fields + constructors | `struct` + `impl` block (`Task::new(...)`) | No class inheritance (`extends`); composition + traits ([src/models.rs](src/models.rs)) |
| **Boilerplate Generation** | Lombok (`@Data`, `@EqualsAndHashCode`) / `record` | `#[derive(Debug, Clone, PartialEq, Eq)]` | Zero-reflection compile-time macros ([src/models.rs](src/models.rs)) |
| **Enums & Tagged Unions** | `enum` or Java 17+ `sealed interface` + `record`s | `enum` (Algebraic Data Types) + exhaustive `match` | Each Rust enum variant can carry different data ([src/models.rs](src/models.rs)) |
| **Interfaces & Generics** | `interface` + `<T extends Summarizable>` (Type Erasure) | `trait` + `<T: Summarizable>` (Monomorphization) | Zero-cost static dispatch; implement traits on any type ([src/models.rs](src/models.rs)) |
| **Memory Management** | Garbage Collector (GC) + heap references | **Ownership, Move Semantics & Borrowing (`&T`, `&mut T`)** | Deterministic cleanup (RAII / `Drop`), zero GC pauses ([src/models.rs](src/models.rs)) |
| **Strings** | `String` (heap object) / `StringBuilder` | `String` (owned heap buffer) vs. `&str` (borrowed view/slice) | Pass `&str` to read strings without allocating ([src/models.rs](src/models.rs)) |
| **Equality** | `a == b` (reference address!) vs. `a.equals(b)` | `a == b` calls `PartialEq` (value equality by default!) | No accidental reference-equality bugs ([src/models.rs](src/models.rs)) |
| **Nullability** | `null` reference (`NullPointerException`) / `Optional<T>` | **No `null`!** `Option<T>` (`Some(T)` / `None`) | Zero-cost Null-Pointer Optimization ([src/exercises.rs](src/exercises.rs)) |
| **Error Handling** | `try / catch / throws` (stack unwinding exceptions) | `Result<T, E>` (`Ok(T)` / `Err(E)`) + `?` operator | Errors are return values; `From` converts error types automatically ([src/storage.rs](src/storage.rs)) |
| **Resource Cleanup** | `try-with-resources` (`AutoCloseable`) | **RAII (`Drop` trait)** | Files and locks close automatically at scope exit `}` ([src/storage.rs](src/storage.rs)) |
| **Streams & Lambdas** | `list.stream().filter(x -> ...).toList()` | `slice.iter().filter(\|x\| ...).collect()` + Lifetimes (`'a`) | Compiles to a raw loop with zero intermediate heap allocations ([src/search.rs](src/search.rs)) |
| **Concurrency** | `Thread`, `synchronized`, `BlockingQueue` | `thread::scope`, `mpsc::channel`, `Arc<Mutex<T>>`, `Send`/`Sync` | Data races are **compile-time errors**; mutex wraps the data ([src/concurrency.rs](src/concurrency.rs)) |
| **Runtime Polymorphism** | Interface reference (`TaskExporter exp`) / `List<TaskExporter>` | Trait Objects: `Box<dyn TaskExporter>` / `&dyn TaskExporter` | Explicit 2-word fat pointer `(data_ptr, vtable_ptr)` ([src/java_patterns.rs](src/java_patterns.rs)) |
| **Constructor Overloading** | Overloaded constructors (`Task(id, title)`, `Task(...)`) | **Builder Pattern** (`TaskBuilder`) + `impl Into<String>` | Rust has no method/constructor overloading ([src/java_patterns.rs](src/java_patterns.rs)) |
| **Custom Destructors** | `AutoCloseable.close()` + `try (...)` / `finally` | `impl Drop for AuditGuard` (`fn drop(&mut self)`) | Compiler runs `drop()` automatically at scope exit `}` ([src/java_patterns.rs](src/java_patterns.rs)) |
| **Functional Interfaces** | `Predicate<T>`, `Consumer<T>` ("effectively final" capture) | `Fn`, `FnMut`, `FnOnce` closure traits | `FnMut` mutates local stack variables directly without `AtomicInteger` ([src/java_patterns.rs](src/java_patterns.rs)) |
| **TreeMap / HashSet / Collectors** | `TreeMap`, `HashSet`, `Collectors.toSet()` | `BTreeMap` (`Ord`), `HashSet` (`Eq + Hash`), polymorphic `.collect()` | `.collect()` can even transpose `Iterator<Result<T, E>>` into `Result<Vec<T>, E>` ([src/java_patterns.rs](src/java_patterns.rs)) |
| **Single-Thread Object Graphs** | Multiple fields referencing & mutating the same `List` | `Rc<RefCell<T>>` (Reference Counting + Interior Mutability) | Moves borrow checking from compile time to runtime ([src/java_patterns.rs](src/java_patterns.rs)) |

---

## Your 7-Step Guided Code Tour

Read the source files in this order to see every `☕ JAVA SIMILARITY` and `🦀 RUST DIFFERENCE` comment in context:

1. **[src/lib.rs](src/lib.rs) — Modules, Crates & Visibility**
   - How `mod` and `use` replace Java `package` and `import`, and why Rust defaults to private visibility.
2. **[src/models.rs](src/models.rs) — Enums, Structs, Ownership, Borrowing & Traits**
   - `Priority` & `Status` enums vs. Java `enum` and `sealed interface`.
   - `Task` struct: ownership transfers (`move`), immutable borrows (`&self`), exclusive mutable borrows (`&mut self`), `&str` vs. `String`, and `Vec<T>` vs. `ArrayList<T>`.
   - `Summarizable` trait & generic `format_badge<T: Summarizable>` vs. Java `interface` and type-erased generics.
3. **[src/storage.rs](src/storage.rs) — `Result<T, E>`, the `?` Operator & RAII File I/O**
   - How `Result<T, E>`, `From<io::Error>`, and `?` replace Java checked exceptions and `try / catch / throws`.
   - Why Rust doesn't need `try-with-resources` (`AutoCloseable`).
4. **[src/search.rs](src/search.rs) — Lifetimes (`'a`), Iterators & Closures vs. Java Streams**
   - Why returning borrowed references (`Vec<&'a Task>`) requires lifetime annotations instead of a Garbage Collector.
   - How `.iter().filter(|t| ...).collect()` compares to Java `Stream`.
5. **[src/concurrency.rs](src/concurrency.rs) — Fearless Concurrency: Message Passing & `Arc<Mutex<T>>`**
   - Pattern 1: Scoped worker threads (`thread::scope`) + channels (`mpsc::channel`).
   - Pattern 2: Shared mutable state with `Arc<Mutex<HashMap<...>>>` vs. Java `synchronized` and GC references.
6. **[src/exercises.rs](src/exercises.rs) & [src/main.rs](src/main.rs) — `Option<T>`, `Comparator` Sorting & CLI Macros**
   - Why `Option<&mut Task>` eliminates `NullPointerException` with zero memory overhead.
   - Chaining `.cmp().then_with()` vs. Java `Comparator.comparing(...).thenComparing(...)`.
7. **[src/java_patterns.rs](src/java_patterns.rs) — Advanced Core Java Patterns in Rust**
   - Dynamic dispatch (`Box<dyn TaskExporter>`), Builder pattern + `impl Into<String>`, custom `Drop` guard vs. `AutoCloseable`, `Fn` / `FnMut` / `FnOnce` vs. "effectively final" lambdas, `BTreeMap` / `HashSet` / polymorphic `.collect()`, and `Rc<RefCell<T>>` interior mutability.

---

## Quick Start Commands

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

# 6. Export tasks using dynamic dispatch (`Box<dyn TaskExporter>`)
cargo run -- export markdown
cargo run -- export csv

# 7. Compute stats across worker threads
cargo run -- stats

# 8. Run all built-in unit tests
cargo test
```
