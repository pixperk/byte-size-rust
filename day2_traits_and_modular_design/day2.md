# 🦀 Day 2: SysInfo CLI - A Rust Project Showcasing Traits, Modular Design & Clap

## Project Overview

**SysInfo CLI** is a sleek, modular command-line tool that displays system information including OS details, Rust version, hostname, and CPU specifications. You can customize the output by selecting exactly which pieces of information you want to see using simple command-line flags.

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/rainbow.png)

## 🔍 How It Works

This project demonstrates several key Rust concepts:

* **Each information type** lives in its own module (`os.rs`, `rust.rs`, `hostname.rs`, `cpu.rs`)
* **All modules implement** a common `InfoProvider` trait, which defines a standard interface:

```rust
pub trait InfoProvider {
    fn key(&self) -> &'static str;   // Returns a label like "CPU"
    fn value(&self) -> String;       // Returns the actual data like "AMD Ryzen 7 (16 cores)"
}
```

* **Command-line parsing** is handled elegantly by the `clap` crate
* **Information display** is toggled with simple flags like `--os` or `--cpu`

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/solar.png)

## 🧩 Understanding Traits in Rust

### What are Traits?

Traits are Rust's approach to **shared behavior** across different types — similar to interfaces in other languages, but more powerful. They allow you to define methods that types must implement to satisfy the trait.

### Why Traits Matter in This Project

In our SysInfo CLI:

* The `InfoProvider` trait creates a **contract**: "Any type implementing this trait must provide a `key()` and a `value()` method"
* This enables **polymorphism**: We can treat different information providers uniformly
* It creates **clean abstractions**: The main code doesn't need to know the implementation details of each provider

### Example Implementation

Here's how our `OsInfo` type implements the trait:

```rust
impl InfoProvider for OsInfo {
    fn key(&self) -> &'static str {
        "OS"
    }

    fn value(&self) -> String {
        let os_name = System::name().unwrap_or_else(|| "Unknown".into());
        let os_version = System::os_version().unwrap_or_else(|| "N/A".into());
        format!("{os_name} {os_version}")
    }
}
```

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/fire.png)

## 🔄 Dynamic Dispatch: The Magic Behind Our Design

### How We Store Different Types Together

In our main code, we use a vector to store all our information providers:

```rust
let providers: Vec<(Box<dyn InfoProvider>, bool)> = vec![
    (Box::new(OsInfo), args.os),
    (Box::new(RustInfo), args.rust),
    (Box::new(HostnameInfo), args.hostname),
    (Box::new(CpuInfo), args.cpu),
];
```

### Breaking It Down

* `Box<dyn InfoProvider>` creates a **trait object**:
  * `Box<>` provides heap allocation, giving us a fixed-size reference
  * `dyn` indicates we're using **dynamic dispatch**
  * `InfoProvider` is the trait our types implement

### Why Dynamic Dispatch?

Dynamic dispatch means the exact method to call is determined **at runtime** rather than compile time:

1. At compile time, Rust doesn't know which concrete type's method will be called
2. At runtime, the program looks up the correct implementation in a virtual method table (vtable)
3. This flexibility allows us to store different types in the same collection

This is slightly less efficient than static dispatch but provides greater flexibility in our design.

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/vintage.png)

## 🎮 Using Clap for Beautiful Command-Line Interfaces

### What is Clap?

`clap` (Command Line Argument Parser) is a powerful Rust crate that generates a complete CLI parser from struct definitions, with minimal boilerplate.

### Our Implementation

We define our CLI interface with a simple struct:

```rust
#[derive(Parser, Debug)]
#[command(author, version, about = "Lessgo Info CLI")]
struct Cli {
    #[arg(long, help = "Show OS info")]
    os: bool,

    #[arg(long, help = "Show Rust version")]
    rust: bool,

    #[arg(long, help = "Show Hostname")]
    hostname: bool,

    #[arg(long, help = "Show CPU info")]
    cpu: bool,
}
```

### How to Use the CLI

Run the program with specific flags to see only what you want:

```bash
# Show only OS and CPU info
cargo run -- --os --cpu

# Show all information (no flags)
cargo run

# See help and available options
cargo run -- --help
```

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

## 🏗️ The Beauty of Modular Design

### Benefits of Our Architecture

Our project's modular design offers several advantages:

* **Extensibility**: Add new information providers by simply creating a new module and implementing the trait
* **Separation of Concerns**: Each module focuses on one specific type of information
* **Maintainability**: Change implementations without affecting the rest of the codebase
* **Testability**: Test each provider in isolation with unit tests

### Code Organization

```
src/
├── main.rs        # Entry point and CLI handling
├── traits.rs      # InfoProvider trait definition
└── info/          # Directory for all info providers
    ├── mod.rs     # Module exports
    ├── os.rs      # OS information provider
    ├── rust.rs    # Rust version provider
    ├── hostname.rs # Hostname provider
    └── cpu.rs     # CPU information provider
```

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/rainbow.png)

## 📝 Quick Recap

* **Traits**: Define shared behavior across different types
* **Dynamic Dispatch**: Call methods on different types through a common interface
* **Clap**: Effortlessly create robust CLI interfaces
* **Modular Design**: Separate concerns for better maintainability and extensibility

![Divider](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/solar.png)

## 🚀 Future Enhancements

* Implement **static dispatch** with generics for potential performance improvements
* Add more information providers:
  * Memory usage stats
  * Network interface details
  * Disk space information
* Support multiple output formats:
  * JSON
  * YAML
  * CSV
* Add visualization options for system monitoring

---

*This project demonstrates fundamental Rust concepts in a practical, hands-on application. By exploring traits, dynamic dispatch, and modular design, we gain valuable insights into Rust's powerful type system and design patterns.*
