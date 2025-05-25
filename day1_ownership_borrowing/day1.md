# Day 1:  Rust Ownership and Borrowing Through a Hotel Management System

This repository demonstrates Rust's ownership and borrowing concepts through a practical hotel management system. The code showcases how Rust's memory safety guarantees work in practice, preventing common programming errors like use-after-free, double-free, and data races at compile time.

## Table of Contents

- [Overview](#overview)
- [The Hotel System Structure](#the-hotel-system-structure)
- [Ownership in Action](#ownership-in-action)
- [Borrowing Mechanisms](#borrowing-mechanisms)
- [Lifetimes and References](#lifetimes-and-references)
- [Key Concepts Explained](#key-concepts-explained)
- [Running the Code](#running-the-code)

## Overview

Rust's ownership system is built on three fundamental rules:
1. Each value in Rust has a single owner
2. When the owner goes out of scope, the value is dropped
3. There can only be one owner at a time

Our hotel management system perfectly illustrates these concepts through guest check-ins, room management, and various ways to access guest information.

## The Hotel System Structure

### Core Data Structures

```rust
struct Guest {
    name: String,
    nights: u32,
}

struct Hotel {
    rooms: HashMap,
}
```

The `Guest` struct owns a `String` (the name) and contains a `u32` value for nights. The `Hotel` struct owns a `HashMap` that maps room numbers to `Guest` instances. This creates a clear ownership hierarchy: Hotel → HashMap → Guest → String.

### Hotel Implementation

The `Hotel` implementation demonstrates different ownership and borrowing patterns through its methods:

## Ownership in Action

### Taking Ownership: `check_in` Method

```rust
fn check_in(&mut self, room_number: u32, guest: Guest) {
    self.rooms.insert(room_number, guest);
}
```

The `check_in` method demonstrates **ownership transfer**:
- It takes ownership of both `room_number` (which is `Copy`, so it's actually copied) and `guest`
- The `guest` parameter moves into the function, transferring ownership from the caller
- When `insert` is called, ownership of the `guest` transfers to the `HashMap`
- After check-in, the original `guest` variable in `main()` is no longer valid

**In main():**
```rust
hotel.check_in(101, Guest {
    name: String::from("Alice"),
    nights: 3,
});
// The Guest instance is now owned by the hotel's HashMap
// We cannot access the original Guest struct anymore
```

## Borrowing Mechanisms

### Immutable Borrowing: `peek_guest` Method

```rust
fn peek_guest(&self, room: u32) -> Option {
    self.rooms.get(&room)
}
```

This method demonstrates **immutable borrowing**:
- Takes an immutable reference to `self` (`&self`)
- Returns an `Option` - a reference to the guest, not the guest itself
- Multiple immutable borrows can exist simultaneously
- The guest data remains owned by the hotel

**Usage:**
```rust
if let Some(guest) = hotel.peek_guest(101) {
    println!("Room 101: {} staying for {} nights", guest.name, guest.nights);
}
// The borrow ends here, guest reference is no longer valid
```

### Mutable Borrowing: `extend_stay` Method

```rust
fn extend_stay(&mut self, room: u32, extra_nights: u32) -> Option {
    if let Some(guest) = self.rooms.get_mut(&room) {
        guest.nights += extra_nights;
        Some(guest)
    } else {
        None
    }
}
```

This method showcases **mutable borrowing**:
- Takes a mutable reference to `self` (`&mut self`)
- Uses `get_mut()` to obtain a mutable reference to the guest
- Modifies the guest's data through the mutable reference
- Returns an immutable reference to the modified guest
- Only one mutable borrow can exist at a time

## Lifetimes and References

### String Slice Borrowing: `guest_report` Method

```rust
fn guest_report(&self, room: u32) -> Option {
    self.rooms.get(&room).map(|g| g.name.as_str())
}
```

This method demonstrates **lifetime relationships**:
- Returns `Option` - a string slice borrowed from the guest's name
- The returned reference has the same lifetime as the hotel instance
- `as_str()` creates a string slice that borrows from the owned `String`
- The borrow is valid as long as the hotel exists

**Usage:**
```rust
if let Some(name) = hotel.guest_report(101) {
    println!("Guest in room 101 is still: {}", name);
}
// The string slice borrow ends here
```

## Key Concepts Explained

### Ownership Transfer vs. Borrowing

**Ownership Transfer (Move):**
- The `check_in` method takes ownership of the `Guest`
- After the move, the original variable cannot be used
- The hotel becomes responsible for the guest's memory

**Borrowing:**
- `peek_guest` and `guest_report` borrow data without taking ownership
- The hotel retains ownership while allowing temporary access
- Multiple immutable borrows can coexist
- Only one mutable borrow can exist at a time

### The Borrow Checker in Action

The commented-out code at the bottom demonstrates what the borrow checker prevents:

```rust
// Move attempt (illegal - try uncommenting)
// let moved_guest = hotel.rooms.remove(&102).unwrap();
// println!("Moved out: {}", moved_guest.name);
```

If uncommented, this would:
1. Move the guest out of the hotel's ownership
2. Make any existing references to that guest invalid
3. Potentially cause use-after-free errors in other languages

Rust's borrow checker prevents this at compile time, ensuring memory safety.

### Memory Safety Guarantees

This system provides several safety guarantees:

1. **No Use-After-Free**: Once a guest is moved, it cannot be accessed through the old reference
2. **No Double-Free**: Each guest is owned by exactly one location
3. **No Data Races**: Mutable and immutable borrows cannot coexist
4. **Automatic Cleanup**: When the hotel goes out of scope, all guests are automatically cleaned up

### Borrowing Rules in Practice

The code demonstrates Rust's borrowing rules:

1. **Rule 1**: You can have either one mutable reference OR any number of immutable references
2. **Rule 2**: References must always be valid (no dangling pointers)
3. **Rule 3**: The borrow checker ensures these rules at compile time

## Running the Code

To run this example:

```bash
cargo run
```

Expected output:
```
Room 101: Alice staying for 3 nights
Guest in room 101 is still: Alice
```

Try uncommenting the last section to see how the Rust compiler prevents unsafe operations!

## Conclusion

This hotel management system elegantly demonstrates how Rust's ownership and borrowing system works in practice. By understanding these concepts through concrete examples, you can write memory-safe code that prevents entire classes of bugs common in other systems programming languages.

The ownership system might seem restrictive at first, but it provides powerful guarantees that eliminate memory safety issues while maintaining zero-cost abstractions and excellent performance.

---
