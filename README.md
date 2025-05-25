# Byte-Size Rust

Welcome to **Byte-Size Rust**, a daily micro-project series where I build and share a tiny Rust project every day. This initiative is designed to foster consistent practice, deepen my understanding of Rust, and demonstrate how even small code snippets can teach powerful concepts.

---

## Table of Contents

- [About This Series](#about-this-series)
- [Why Byte-Size Projects?](#why-byte-size-projects)
- [How to Run](#how-to-run)
- [What I Learned Today](#what-i-learned-today)
- [Next Steps](#next-steps)

---

## About This Series

**Byte-Size Rust** is my commitment to building one small Rust project every day. Each project is intentionally kept minimal—often just a few lines or a single file—yet is focused on exploring a specific Rust feature, idiom, or best practice. This approach is inspired by the idea that frequent, focused practice is the fastest way to mastery[2].

---


## Why Byte-Size Projects?

- **Focused Learning**: Small projects help isolate and understand one concept at a time[2].
- **Consistency**: Daily practice builds habits and confidence.
- **Shareable**: Each project is easy to review, remix, and share with others.
- **Foundation for Bigger Ideas**: These micro-projects can become building blocks for larger applications.

---

## How to Run

1. Ensure you have [Rust installed](https://www.rust-lang.org/tools/install).
2. Clone this repository or copy the code for today's project.
3. Run the project using Cargo:

   ```bash
   cargo run
   ```

4. You should see output demonstrating the ownership and borrowing mechanics in action.

---

## What I Learned Today

- **Ownership**: Values have a single owner in Rust; moving them transfers responsibility.
- **Borrowing**: References allow safe access to data without transferring ownership.
- **Mutability**: Variables are immutable by default; use `mut` for mutability.
- **Lifetimes**: Borrowed data must not outlive its owner, enforced by the borrow checker[1].

These concepts are foundational to writing safe and efficient Rust code.

---

## Next Steps

- Tomorrow, I'll tackle a new Rust concept or library in another byte-size project.

---

> **Follow along or fork this repo to join the challenge!**
>
> Each day is a new opportunity to learn, experiment, and grow as a Rustacean.

---

**Happy coding!**

---
