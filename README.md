# Rust Toy Lisp interpreter
This is a really simple Lisp interpreter, written in Rust. Just for educational purposes. Used to discover how to work with lexing libraries and how to write basic interpreter. Functionality that is done works really solid, but do not expect any new functionality beyond ones that listed in the Features.

Based on v0.0.1 of this [book](https://vishpat.github.io/lisp-rs/overview.html) but with distinct difference in underground structure of lexing and operator evaluation as operators now accept arbitrary number of arguments.

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
- Run: ```cargo build --release```
- Executable is created at ```target/release/lisp-rs```

## Usage
```bash
Usage:
        Start a REPL  lisp-rs
        Execute file  lisp-rs [FILE_PATH]

Main options:
        -h, -?, --help Print this help message and exit
```

## Toy Lisp Examples
Check [lisp-examples](https://github.com/ZAZPRO/toy-lisp-rs/tree/main/lisp-examples) project directory.
