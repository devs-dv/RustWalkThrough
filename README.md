# Learnings of Rust

Welcome to the Learnings of Rust repository! This repository is dedicated to documenting my journey as I learn the Rust programming language. It includes notes, examples, and projects that I've worked on to understand Rust better.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Basic Concepts](#basic-concepts)
- [Projects](#projects)
- [Resources](#resources)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Rust is a systems programming language that focuses on speed, memory safety, and parallelism. This repository contains my personal notes and projects as I learn Rust, which I hope can serve as a resource for others who are also learning the language.

## Installation

To install Rust, follow these steps:

1. Download and install Rust from the official website: [rust-lang.org](https://www.rust-lang.org/).
2. Follow the instructions to set up Rust using `rustup`.
3. Verify the installation by running:
    ```bash
    rustc --version
    ```

## Basic Concepts

### Variables and Mutability

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("The value of y is now: {}", y);
}
```

### Data Types

Rust has several built-in data types, including integers, floating-point numbers, booleans, and characters.

### Functions

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

### Control Flow

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

## Projects

1. **Hello World:** A simple program to print "Hello, world!".
2. **Guessing Game:** A small project to implement a number guessing game.
3. **CLI Calculator:** A command-line calculator for basic arithmetic operations.

## Resources

Here are some useful resources for learning Rust:

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Official Rust Documentation](https://doc.rust-lang.org/)



Happy coding!
