# Lox-rs Programming Language in Rust

This repository provides an implementation of the Lox programming language in Rust, following the guidance from the "Crafting Interpreters" book by Robert Nystrom. Lox is a dynamically-typed, object-oriented programming language that is simple to understand, making it an excellent language for learning about language design and implementation.

## Introduction

Lox is a high-level programming language that supports features such as variables, control flow, functions, classes, and more. It's designed to be a straightforward language for learning about language design and crafting an interpreter. This implementation follows the principles outlined in the "Crafting Interpreters" book, which provides detailed insights into creating a Lox interpreter in two main stages: a tree-walking interpreter and a bytecode compiler.

```js
if (condition) {
    print "yes";
} else {
    print "no";
}

var a = 1;
while (a < 10) {
    print a;
    a = a + 1;
}

for (var a = 1; a < 10; a = a + 1) {
    print a;
}

fun printSum(a, b) {
    print a + b;
}

class Breakfast {
    cook() {
        print "Eggs a-fryin'!";
    }

    serve(who) {
        print "Enjoy your breakfast, " + who + ".";
    }
}
```
## Current State

The Interpreter currently scans, parses, and evaluates expressions as statements with printing. A working example can 
be found here: [./lox/dev.lox](./lox/dev.lox)

## Getting Started

To get started, clone this repository and run the following command:

```bash
cargo build --release
```

This will build the Lox interpreter in release mode, which will be located at `target/release/lox`. You can then run the interpreter with a Lox script as an argument:

```bash
target/release/lox code.lox
```

or you can run the interpreter in REPL mode (interactive mode):

```bash
target/release/lox
```

## Language Features

The language is fully documented by its designer at [https://craftinginterpreters.com/the-lox-language.html](https://craftinginterpreters.com/the-lox-language.html).
