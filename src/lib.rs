//! # RustQuest — Rust for Java Developers
//!
//! ## ☕ JAVA SIMILARITY: Packages vs Modules
//! - In Java, files are organized into `package com.example.rustquest;` and imported with `import`.
//! - In Rust, each file in `src/` is a **module** declared here in `lib.rs` (the library crate root)
//!   and imported using `use crate::models::Task;`.
//!
//! ## 🦀 RUST DIFFERENCE: Visibility & Crates
//! - In Java, class members default to *package-private*.
//! - In Rust, **everything is private by default** (even to sibling modules in the same crate)
//!   unless explicitly marked `pub`.
//! - A **crate** (configured by `Cargo.toml`) is the compilation unit—similar to a Maven/Gradle
//!   module + JAR artifact combined.

pub mod concurrency;
pub mod exercises;
pub mod models;
pub mod search;
pub mod storage;
