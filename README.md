# Rust Learning Journey 🚀

This repository contains my comprehensive Rust learning projects and exercises, completed as part of the TurbinQ1 Builders Cohort preparation. I've worked through **The Rust Book** and implemented various projects to demonstrate proficiency in Rust programming.

## 📚 Learning Overview

### The Rust Programming Language (The Book)
I've completed the core concepts and projects from "The Rust Programming Language" book, including:

- **Chapter 2**: Guessing Game - A number guessing game with user input
- **Core Concepts**: Variables, functions, ownership, borrowing, structs, enums, traits, error handling, and more

### Additional Learning Projects
Beyond The Book, I've built several projects to deepen my understanding of Rust concepts and real-world applications.

## 🗂️ Project Structure

### 📁 Core Projects

#### 1. `guess_number/` - The Classic Guessing Game
**From Chapter 2 of The Rust Book**
- Interactive number guessing game
- Random number generation using `rand` crate
- Input validation and error handling
- Colored output using `colored` crate
- **Key Concepts**: User input, random numbers, pattern matching, loops

```rust
// Core game logic with input validation
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Type Correct number");
        continue;
    },
};
```

#### 2. `todo_cli/` - Command Line Todo Application
**Advanced CLI Application**
- Full-featured todo list manager
- JSON file-based persistence
- Command-line argument parsing with `clap`
- CRUD operations (Create, Read, Update, Delete)
- Unit testing with `assert_cmd`
- **Key Concepts**: File I/O, JSON serialization, CLI design, error handling

**Commands:**
- `todo add "Task description"` - Add new task
- `todo list` - List all tasks
- `todo done <id>` - Mark task as completed
- `todo remove <id>` - Remove task
- `todo clear` - Clear all tasks

#### 3. `03-server/` & `04-tcp-server/` - HTTP Server Implementation
**Networking and HTTP Concepts**
- Custom TCP server implementation
- HTTP request parsing
- Basic routing concepts
- Multi-threaded server design
- **Key Concepts**: Networking, TCP connections, HTTP protocol, request/response handling

**Server Features:**
- TCP listener on localhost:8080
- HTTP request parsing
- Basic response handling
- Connection management

### 📁 Learning Concept Projects

#### 4. `hello/` - Fundamental Rust Concepts
**Comprehensive learning examples covering:**
- Functions and recursion (Fibonacci)
- String manipulation
- Structs and implementations
- Enums and pattern matching
- Basic algorithms

#### 5. `mars_calc/` - Mars Weight Calculator
**Practical application of basic concepts:**
- User input handling
- Mathematical calculations
- Mars gravity conversion (Earth weight → Mars weight)
- Formula: `(earth_weight / 9.81) * 3.711`

#### 6. `traits/` - Trait System Exploration
**Object-oriented concepts in Rust:**
- Defining traits
- Implementing traits for structs
- Associated methods
- Polymorphism concepts

#### 7. `lifetimes/` - Lifetime Concepts
**Rust's ownership system exploration**
- Understanding borrowing and lifetimes
- Memory management concepts

## 🛠️ Technologies & Libraries Used

### Core Rust Features
- **Ownership & Borrowing**: Memory safety without garbage collection
- **Pattern Matching**: Powerful control flow with `match` expressions
- **Error Handling**: `Result` and `Option` types
- **Generics & Traits**: Code reusability and polymorphism
- **Collections**: Vectors, strings, hash maps
- **File I/O**: Reading/writing files and JSON

### External Crates
- **`clap`**: Command-line argument parser
- **`serde` & `serde_json`**: JSON serialization/deserialization
- **`anyhow`**: Error handling
- **`rand`**: Random number generation
- **`colored`**: Terminal color output
- **`assert_cmd`**: CLI application testing


## 🚀 How to Run Projects

### Prerequisites
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

### Running Individual Projects

#### Guess Number Game
```bash
cd guess_number
cargo run
```

#### Todo CLI
```bash
cd todo_cli
cargo build --release
# Add a task
./target/release/todo_cli add "Learn Rust macros"
# List tasks
./target/release/todo_cli list
# Mark task as done
./target/release/todo_cli done 1
```

#### HTTP Server
```bash
cd 03-server/server
cargo run
# Server will start on localhost:8080
```

#### Mars Calculator
```bash
cd 02-mars-calculaor-cmd/mars_calc
cargo run
# Enter your Earth weight when prompted
```

### Running Tests
```bash
cd todo_cli
cargo test
```

## 📈 Learning Progress & Achievements

### ✅ Completed Concepts
- [x] Variables, mutability, and scoping
- [x] Functions and control flow
- [x] Ownership, borrowing, and lifetimes
- [x] Structs, enums, and pattern matching
- [x] Traits and generic types
- [x] Error handling with Result/Option
- [x] Collections (Vectors, Strings, HashMaps)
- [x] File I/O and JSON handling
- [x] Command-line applications
- [x] TCP networking and HTTP basics
- [x] Unit testing and integration testing
- [x] Macros and metaprogramming

### 🔄 Areas for Future Exploration
- [ ] Advanced concurrency (async/await)
- [ ] Web frameworks (Actix, Rocket)
- [ ] Database integration
- [ ] WebAssembly (WASM)
- [ ] Systems programming concepts

## 🎯 Project Highlights

### Most Complex Project: Todo CLI
**Features Implemented:**
- Persistent JSON storage
- Full CRUD operations
- Professional CLI interface with `clap`
- Comprehensive error handling
- Unit tests
- Clean, maintainable code structure

### Networking Project: HTTP Server
**Technical Implementation:**
- Raw TCP socket programming
- HTTP request parsing
- Custom request/response structures
- Multi-client handling
- Error recovery and logging

## 📝 Notes & Reflections

### Key Learning Points
1. **Ownership System**: Rust's unique approach to memory safety through ownership, borrowing, and lifetimes
2. **Zero-Cost Abstractions**: High-level programming without runtime overhead
3. **Pattern Matching**: Powerful control flow that's both expressive and safe
4. **Error Handling**: Explicit error handling that prevents silent failures
5. **Cargo Ecosystem**: Excellent package management and build system

### Challenges Overcome
- Understanding the borrow checker and lifetime annotations
- Implementing proper error handling throughout applications
- Building concurrent TCP server without data races
- Creating testable CLI applications

### Best Practices Learned
- Writing idiomatic Rust code
- Proper error propagation with `?` operator
- Using appropriate data structures
- Writing comprehensive tests
- Documentation and code organization

## 🔗 Resources Used

- **The Rust Programming Language Book**: Primary learning resource
- **Rust Documentation**: Official API documentation
- **crates.io**: Discovering and using external crates
- **Rust Community**: Forums and discussion boards

## 📊 Repository Statistics

- **Total Projects**: 8
- **Lines of Code**: ~1000+ lines across all projects
- **External Dependencies**: 8 crates used
- **Test Coverage**: Basic unit tests implemented

---

**Prepared for TurbinQ1 Builders Cohort Submission**
*Demonstrating completion of The Rust Book projects and core Rust concepts*

*Repository: [rust-learning](https://github.com/gamandeepsingh/rust-learning)*
*Author: Gamandeep Singh*</content>
