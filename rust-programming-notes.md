# Rust For Undergrads Free Course - Udemy

A deep dive into basics of Rust programming language.

## Table of Contents

- [Section 1: Introduction](#section-1-introduction)
- [Section 2: Installatiion](#section-2-installatiion)
- [Section 3: Hello World with Rust](#section-3-hello-world-with-rust)
- [Section 4: Basics Concepts](#section-4-basics-concepts)
- [Section 5: Ownership with Rust](#section-5-ownership-with-rust)
- [Section 6: Data Handling and Pattern Matching in Rust](#section-6-data-handling-and-pattern-matching-in-rust)
- [Section 7: Error Handling](#section-7-error-handling)
- [Section 8: Conclusion](#section-8-conclusion)


## Section 1: Introduction

### Overview 

- Rust is a systems programming language sponsored by Mozilla which describes it as a "safe, concurrent, practical language", supporting functional and imperative-procedural paradigms. 

- Rust is syntactically similar to C++, but its designers intend it to provide **better memory safety** while still maintaining performance prevents segfaults, and guarantees **thread safety**. 

### Prerequisites

- Basic programming experience
- Beginning Programmers
- Advanced Programmers
- Basics Concepts: OOP, ...

## Section 2: Installatiion

- Click on the following link to go to the official installation website: https://www.rust-lang.org/tools/install
- Based on the OS you're using the website assistant will provide you with the installation guide for your machine.

### Update Rust packages & toolchain (after the installation): 

- `rustup`: The Rust toolchain installer
- `cargo`: Rust's package manager
- `rustc`; Rust compiler

Toolchain update + Cargo: 

```rs
rustup update
```

## Section 3: Hello World with Rust


### Hello World!

```rs
fn main() {
    println!("Hello, world!");
}
```

Compile: 

` rustc .\hello_world.rs`

Execute: 

```
\hello_world\hello_world.exe
```

### Hello World using Cargo

Why? 
- Complex Rust programs need dependencies

Automate your compilation process with `cargo`:

- Cargo - the build system and package manager of rust
  - build the code
  - download libraries your code depends on (dependencies)

To create cargo project execute the command below:

```PS
cargo new my_cargo_project --bin
``` 
output:

```PS
Created binary (application) `my_cargo_project` package
```

To build & run the project:

```PS
cargo build
```

Then, 

```PS
cargo run
```

if any error:

```rs
rustc --explain E0382
```
Where `E0382` is the error id

### Quiz

## Section 4: Basics Concepts

- [04 - Hello World](./lab/04-Hello-World/)
- [05 - Hello Cargo](./lab/05-Hello-Cargo/)
- [06 - Variables and Mutability](./lab/06-Variables-and-Mutability/)
- [07 - Scalar Data Type - Integer](./lab/07-Scalar-Data-Type-Integer/)
- [08 - Scalar Data Type - Float](./lab/08-Scalar-Data-Types-Floats/) 
- [09 - Compound Data Type - Tuples](./lab/09-Tuples/)
- [10 - Compound Data Type - Arrays](./lab/10-Arrays/) 
- [11 - Functions](./lab/11-Functions/) 
- [12 - Returning value Functions](./lab/12-Returning-Value-Functions/)
- [13 - if-else](./lab/13-if-else/)
- [14 - While Loops](./lab/14-While-Loops/)
- [15 - For Loops](./lab/15-For-Loops/) 

## Section 5: Ownership with Rust

- [16 - Ownership](./lab/16-Ownership/) 
- [17 - Ownership and Borrowing](./lab/17-Ownership-and-Borrowing/) 
- [18 - Race Conditions](./lab/18-race-conditions/)
- [19 - Slices](./lab/19-Slices/)

## Section 6: Data Handling and Pattern Matching in Rust

- [20 - Structs](./lab/20-Structs/)
- [21 - Structs Example](./lab/21-Structs%20Example/) 
- [22 - Method Syntax](./lab/22-Method%20Syntax/) 
- [23 - Enums](./lab/23-Enums/)
- [24 - Enums with mul types](./lab/24-Enums-With-Mul-types/) 
- [25 - Pattern Matching](./lab/25-Pattern-Matching/)

## Section 7: Error Handling

- [26 - Unrecoverable errors with panic](./lab/26-Unrecoverable-errors-with-panic/) 
- [27 - recoverable errors](./lab/27-recoverable%20errors/)
- [28 - recoverable errors with demo](./lab/28-recoverable-errors-with-demo/)


## Section 8: Conclusion


## References

**Some of functionalities that Rust std Libraries(Colletion of Crates)/Modules/Packages covers:**

```
- Core language features: The standard library provides the basic building blocks of the Rust language, such as integers, floats, strings, and booleans.
- Data structures: The standard library provides a variety of data structures, such as vectors, maps, and sets.
- IO: The standard library provides functions for reading and writing data to and from the filesystem, network sockets, and other sources.
- Concurrency: The standard library provides primitives for writing concurrent code, such as threads, mutexes, and condition variables.
- Error handling: The standard library provides a mechanism for handling errors, such as the Result type.
```

