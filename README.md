# Byte-Size Rust

Welcome to **Byte-Size Rust**, a daily micro-project series where I build and share a tiny Rust project every day. This initiative is designed to foster consistent practice, deepen my understanding of Rust, and demonstrate how even small code snippets can teach powerful concepts.

---

## Table of Contents

- [About This Series](#about-this-series)
- [Why Byte-Size Projects?](#why-byte-size-projects)
- [Project Index](#project-index)
- [How to Run](#how-to-run)
- [Next Steps](#next-steps)

---

## About This Series

**Byte-Size Rust** is my commitment to building one small Rust project every day. Each project is intentionally kept minimal—often just a few lines or a single file—yet is focused on exploring a specific Rust feature, idiom, or best practice. This approach is inspired by the idea that frequent, focused practice is the fastest way to mastery.

---


## Why Byte-Size Projects?

- **Focused Learning**: Small projects help isolate and understand one concept at a time.
- **Consistency**: Daily practice builds habits and confidence.
- **Shareable**: Each project is easy to review, remix, and share with others.
- **Foundation for Bigger Ideas**: These micro-projects can become building blocks for larger applications.

---

## Project Index

Here's a chronological list of all the daily micro-projects in this series:

### [Day 1: Ownership & Borrowing](/day1_ownership_borrowing/)
A hotel management system that demonstrates Rust's ownership, borrowing, and lifetime concepts.
- [Detailed Write-up](/day1_ownership_borrowing/day1.md)
- Key concepts: Ownership transfer, borrowing, mutable references, lifetimes

### [Day 2: Traits & Modular Design](/day2_traits_and_modular_design/)
A system information CLI tool showcasing traits, modules, and the Clap crate for argument parsing.
- [Detailed Write-up](/day2_traits_and_modular_design/day2.md)
- Key concepts: Traits, modules, polymorphism, command-line argument parsing

---

## How to Run

Each project has its own directory with a standalone Rust project:

1. Ensure you have [Rust installed](https://www.rust-lang.org/tools/install).
2. Navigate to the specific day's directory:
   ```bash
   cd day1_ownership_borrowing
   ```
3. Run the project using Cargo:
   ```bash
   cargo run
   ```
4. For projects with command-line arguments (like Day 2), you can pass flags:
   ```bash
   cargo run -- --help  # View available options
   cargo run -- --all   # Show all information
   ```

Each project directory contains a detailed markdown file explaining the concepts and implementation details.

---

## Next Steps

Here are some concepts I plan to explore in upcoming byte-size projects:

- **Error Handling**: Custom error types and the `Result` type
- **Smart Pointers**: `Box<T>`, `Rc<T>`, and `RefCell<T>`
- **Concurrency**: Threads, message passing, and shared state
- **Async/Await**: Non-blocking I/O with Rust's async ecosystem
- **Advanced Traits**: Associated types, default implementations, and trait objects
- **Macros**: Procedural and declarative macros for code generation

---

> **Follow along or fork this repo to join the challenge!**
>
> Each day is a new opportunity to learn, experiment, and grow as a Rustacean.

---

**Happy coding!**

---
