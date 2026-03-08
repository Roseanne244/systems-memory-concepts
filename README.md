# 🦀 Rust Learning Notes

Personal documentation of my Rust programming journey — from zero to systems & Web3 development.

---

## 📚 Table of Contents

- [Why Rust?](#why-rust)
- [Installation](#installation)
- [Core Concepts](#core-concepts)
- [Ownership & Borrowing](#ownership--borrowing)
- [Data Types](#data-types)
- [Control Flow](#control-flow)
- [Functions & Closures](#functions--closures)
- [Structs & Enums](#structs--enums)
- [Error Handling](#error-handling)
- [Traits](#traits)
- [Collections](#collections)
- [Concurrency](#concurrency)
- [Resources](#resources)

---

## 🤔 Why Rust?

Rust is a **systems programming language** focused on:
- **Safety** — No null pointers, no data races, memory safe without garbage collector
- **Speed** — As fast as C/C++
- **Concurrency** — Fearless concurrency via ownership system

Popular use cases:
- Web backends (fast, low-memory)
- CLI tools
- WebAssembly
- Blockchain / Smart contracts (Solana, Near)
- Embedded systems
- Game engines

---

## ⚙️ Installation

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify
rustc --version
cargo --version
```

**Key tools:**
| Tool | Purpose |
|------|---------|
| `rustc` | Rust compiler |
| `cargo` | Package manager + build tool |
| `rustup` | Rust version manager |
| `clippy` | Linter |
| `rustfmt` | Code formatter |

---

## 🧠 Core Concepts

### Hello World
```rust
fn main() {
    println!("Hello, World!");
}
```

### Variables & Mutability
```rust
let x = 5;           // immutable by default
let mut y = 10;      // mutable with `mut`
y = 20;              // OK
// x = 6;           // ERROR: cannot assign twice to immutable variable

const MAX: u32 = 100_000; // constant, must have type annotation
```

### Shadowing
```rust
let x = 5;
let x = x + 1;       // new variable, shadows old one
let x = x * 2;       // x is now 12
```

---

## 🔑 Ownership & Borrowing

This is Rust's most unique feature — it replaces garbage collection.

### Rules:
1. Each value has exactly **one owner**
2. When the owner goes out of scope, the value is **dropped**
3. You can have **either** one mutable reference **or** multiple immutable references

```rust
// Ownership move
let s1 = String::from("hello");
let s2 = s1;           // s1 is MOVED to s2
// println!("{}", s1); // ERROR: s1 no longer valid

// Clone (deep copy)
let s1 = String::from("hello");
let s2 = s1.clone();   // explicit deep copy
println!("{} {}", s1, s2); // both valid

// Borrowing (immutable reference)
let s1 = String::from("hello");
let len = calculate_length(&s1); // pass reference, not value
println!("{} has {} chars", s1, len); // s1 still valid

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Mutable reference
let mut s = String::from("hello");
change(&mut s);

fn change(s: &mut String) {
    s.push_str(", world");
}
```

---

## 📊 Data Types

### Scalar Types
```rust
// Integers
let a: i32 = -42;
let b: u64 = 1_000_000;

// Floats
let f: f64 = 3.14;

// Boolean
let t: bool = true;

// Character
let c: char = '🦀';
```

### Compound Types
```rust
// Tuple
let tup: (i32, f64, bool) = (42, 3.14, true);
let (x, y, z) = tup;       // destructuring
let first = tup.0;          // index access

// Array (fixed size)
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 10];        // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

// String types
let s1 = "hello";           // &str (string slice, immutable)
let s2 = String::from("hello"); // String (heap-allocated, mutable)
```

---

## 🔀 Control Flow

```rust
// if / else if / else
let number = 7;
if number < 5 {
    println!("less than 5");
} else if number == 7 {
    println!("seven!");
} else {
    println!("greater than 5");
}

// if as expression
let result = if number > 5 { "big" } else { "small" };

// loop
let mut count = 0;
let result = loop {
    count += 1;
    if count == 10 { break count * 2; }
};

// while
let mut n = 3;
while n > 0 {
    println!("{n}!");
    n -= 1;
}

// for
for i in 1..=5 {       // 1 to 5 inclusive
    println!("{i}");
}

let arr = [10, 20, 30];
for element in arr.iter() {
    println!("{element}");
}
```

---

## ⚡ Functions & Closures

```rust
// Function
fn add(x: i32, y: i32) -> i32 {
    x + y  // no semicolon = return value (expression)
}

// Closure (anonymous function)
let multiply = |x: i32, y: i32| x * y;
println!("{}", multiply(3, 4)); // 12

// Closure capturing environment
let base = 10;
let add_base = |x| x + base;
println!("{}", add_base(5)); // 15

// Higher-order functions
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
```

---

## 🏗️ Structs & Enums

```rust
// Struct
struct User {
    name: String,
    email: String,
    age: u32,
    active: bool,
}

impl User {
    // Constructor (associated function)
    fn new(name: &str, email: &str, age: u32) -> User {
        User {
            name: String::from(name),
            email: String::from(email),
            age,
            active: true,
        }
    }

    // Method
    fn greet(&self) {
        println!("Hi, I'm {}!", self.name);
    }
}

let user = User::new("Roseanne", "rose@example.com", 25);
user.greet();

// Enum
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with data (like a tagged union)
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

// Match on enum
let msg = Message::Move { x: 10, y: 20 };
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({x}, {y})"),
    Message::Write(text) => println!("Write: {text}"),
    Message::Color(r, g, b) => println!("Color: {r},{g},{b}"),
}
```

---

## ⚠️ Error Handling

```rust
use std::fs;

// Option<T> — value might not exist
fn find_user(id: u32) -> Option<String> {
    if id == 1 { Some(String::from("Roseanne")) } else { None }
}

match find_user(1) {
    Some(name) => println!("Found: {name}"),
    None => println!("Not found"),
}

// Shorthand
let name = find_user(1).unwrap_or(String::from("Guest"));

// Result<T, E> — operation might fail
fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

match read_file("data.txt") {
    Ok(content) => println!("{content}"),
    Err(e) => println!("Error: {e}"),
}

// ? operator — propagate error up
fn process() -> Result<String, std::io::Error> {
    let content = fs::read_to_string("data.txt")?; // returns Err if fails
    Ok(content.to_uppercase())
}
```

---

## 🎭 Traits

```rust
// Trait = interface / abstract class
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    fn describe(&self) {  // default implementation
        println!("I am {} and I say {}", self.name(), self.sound());
    }
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn name(&self) -> &str { "Dog" }
    fn sound(&self) -> &str { "Woof" }
}

impl Animal for Cat {
    fn name(&self) -> &str { "Cat" }
    fn sound(&self) -> &str { "Meow" }
}

let dog = Dog;
dog.describe(); // I am Dog and I say Woof
```

---

## 📦 Collections

```rust
// Vector (dynamic array)
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);

let v2 = vec![1, 2, 3, 4, 5]; // macro shorthand

// HashMap
use std::collections::HashMap;
let mut scores: HashMap<String, u32> = HashMap::new();
scores.insert(String::from("Alice"), 100);
scores.insert(String::from("Bob"), 85);

let alice_score = scores.get("Alice"); // Option<&u32>
```

---

## 🔄 Concurrency

```rust
use std::thread;
use std::sync::{Arc, Mutex};

// Spawn threads
let handle = thread::spawn(|| {
    println!("Hello from thread!");
});
handle.join().unwrap();

// Shared state with Arc + Mutex
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let h = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(h);
}

for h in handles { h.join().unwrap(); }
println!("Counter: {}", *counter.lock().unwrap()); // 10
```

---

## 📖 Resources

| Resource | Link |
|----------|------|
| The Rust Book (free) | [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) |
| Rustlings (exercises) | [github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings) |
| Rust by Example | [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/) |
| Exercism Rust Track | [exercism.org/tracks/rust](https://exercism.org/tracks/rust) |
| Awesome Rust | [github.com/rust-unofficial/awesome-rust](https://github.com/rust-unofficial/awesome-rust) |

---

## 📅 Learning Progress

- [x] Understand ownership & borrowing
- [x] Learn basic syntax and data types
- [x] Understand error handling with Option & Result
- [x] Learn structs, enums, and traits
- [ ] Complete Rustlings exercises
- [ ] Build a CLI tool
- [ ] Build a REST API with Axum
- [ ] Build a Solana program

---
*Living document — updated as I learn. Feel free to fork!*
