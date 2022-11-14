# Rust Toy Lisp interpreter
Based on v0.0.1 of this [book](https://vishpat.github.io/lisp-rs/overview.html) but with distinct differences.

## Lisp Features
- Integers
- Floats
- Variable definitions
- Conditionals
- Lambdas
- Arbitrary number of arguments for operators
- Advanced parsing using [logos](https://crates.io/crates/logos)

## Interface Features
- Advanced REPL with history using [rustyline](https://crates.io/crates/rustyline)
- File Evaluation

## Installation
- Clone repository
- cargo build --release
- Executable is created at target/release/lisp-rs

## Usage
### REPL
Just run lisp-rs without arguments.

```bash
lisp-rs
```

### Evaluate a source file
Just provide a path to a source file as an argument

```bash
lisp-rs PATH_TO_LISP_SOURCE
```

## Toy Lisp Examples
Check lisp-examples folder
