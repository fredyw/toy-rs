# toy-rs

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![toy-rs](https://github.com/fredyw/toy-rs/actions/workflows/toy-rs.yml/badge.svg)](https://github.com/fredyw/toy-rs/actions/workflows/toy-rs.yml)

A toy programming language with Rust-like syntax written in Rust.

### Usage

```
toy-rs <filemame.toy>
```

### Syntax

`toy-rs` supports a subset of Rust-like syntax.

#### Variables

Variables are declared using the `let` keyword.

```rust
let x = 10;
let y = 20.5;
let message = "Hello";
let is_valid = true;
```

#### Assignments

Standard and compound assignment operators are supported.

```rust
let x = 10;
x += 5; // 15
x -= 2; // 13
x *= 2; // 26
x /= 2; // 13
```

#### Data Types

- **Integers**: `1`, `42`, `-10`
- **Floats**: `3.14`, `0.5`, `-2.0`
- **Booleans**: `true`, `false`
- **Strings**: `"Hello World"`

#### Arithmetic Operations

Standard arithmetic operators are supported for Integers and Floats. Mixed-type arithmetic (e.g., Int + Float) is supported and results in a Float.

```rust
let sum = 5 + 10;
let product = 2.5 * 4;
let mixed = 10 + 2.5;
```

#### Logical Operations

Logical AND (`&&`) and OR (`||`) operators are supported. `&&` has higher precedence than `||`.

```rust
let valid = true && (false || true); // true
let check = 1 < 2 && 3 > 2; // true
```

#### Functions
Functions are declared using `fn`. The last expression in a block or a function body is implicitly returned.

```rust
fn add(a, b) {
    a + b
}

let result = add(10, 20);
```

#### Control Flow

`if` and `else` expressions are supported. They return the value of the branch that was executed.

```rust
let x = 10;
let status = if x > 5 {
    "Greater"
} else {
    "Smaller"
};
```

#### Loops

`while` loops are supported for repeated execution based on a boolean condition.

```rust
let i = 0;
while i < 5 {
    i += 1;
}
```

#### Comments

Single-line comments starting with `//` are supported.

```rust
// This is a comment
let x = 5; // Inline comment
```

### Building

To build the project, you need to have Rust installed. You can install it from [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can build the project by running the following command:

```
./build.sh --release
```

The binary will be located in `target/release/toy-rs`.

### Installing

To install `toy-rs`, you can use the following command.

```
./install.sh
```

### Testing

To run the tests, you can use the following command.

```
./test.sh
```
