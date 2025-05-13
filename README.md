# Rust: The Complete Developer's Guide

Rust: The Complete Developer's Guide by Stephen Grider

## Folder structure

- 01-foundations/deck
  - section 1 and 2
- 03-rust-memory-system
  - bank
    - section 3 and 4
  - comparison-js-rust: comparing javascript and rust memory system
- 05-enums
  - media
    - section 5 and 6
- 07-errors-results
  - logs
    - section 7

## Details

<details open>
  <summary>Click to Contract/Expend</summary>

## Section 1: Foundations of Rust: Setup and First Steps

### 2. Rust Installation

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo --version
# cargo 1.86.0 (adf9b6ad1 2025-02-28)
```

### 4. Creating and Running Rust Projects

- `cargo new <project name>`
- `cargo runs`

```sh
mkdir 01-foundations
cd 01-foundations
cargo new deck
```

```sh
cd deck
cargo run
cargo run -q  # without debugging messages
```

## Section 2: Core Concepts: The Building Blocks of Rust

### 7. Representing Data with Structs

- `struct` = class
- `let` is not actually variable, but binding. Binding is immutable

### 10. Mutable vs Immutable Bindings

```rust
// good debugging code with formatting
println!("Heres your deck: {:#?}", deck);
```

### 13. Installing External Crates

- `crate` = package

#### Rust Standard Library

- Included with every project without any additional install
- Docs at `https://doc.rust-lang.org/stable/std/`

#### External Crates

- Have to be install into our project with `cargo add <crate name>`
- Crate listing at `https://crates.io/`
- Docs also at `https://docs.rs/`

```sh
# 01-foundations/deck
cargo add rand
```

## Section 3: Ownership and Borrowing: Rust's Unique Memory System

### 19. Project Overview

```sh
mkdir 03-rust-memory-system
cd 03-rust-memory-system
cargo new bank
cd bank
```

### 21. A Mysterious Error

```rust
fn main() {
    let account = Account::new(1, String::from("Noah"));

    print_account(account);
    print_account(account); // it gets the mysterious error
}
```

### 22. Unexpected Value Updates

#### Ownership

1. Every value is 'owned' by a **single variable**, struct, vector, etc at a time

2. Reassigning the value to another variable, passing it to a function, putting it into a vector, etc, **moves** the value. The old variable can't be used anymore!

#### Borrowing

3. You can create many read-only references to a value that exist at the same time

4. You can't move a value while a ref to the value exists

5. You can make a writeable (mutable) reference to a value only if there are no read-only references currently in use. One mutable ref to a value can exist at a time

6. You can't mutate a value through the owner when any ref (mutable or immutable) to the value exists

7. Some types of values are copied instead of moved (numbers, bools, chars, arrays/tuples with copyable elements) - **break rules of ownership (=it works as similar as usual programming languages)**

#### Lifetimes

8. When a variable goes out of scope, the value owned by it is dropped (cleaned up in memory)

9. Values can't be dropped if there are still active references to it

10. References to a value can't outlive the value they refer to

#### So

11. These rules will dramatically change how you write code (compared to other languages)

**12. When in doubt, remember that Rust wants to minimize unexpected updates to data**

### 23. The Goal of Ownership and Borrowing

- Lesson 1. make the engine read-only
- Lesson 2. each objects has their own properties

### 24. The Basics of Ownership

> Above all, Rust wants to avoid 'unexpected updates'

## Section 5: Enums Unleashed: Pattern Matching and Options

### 45. Project Overview

```sh
mkdir 05-enums
cd 05-enums
cargo new media
cd media
```

### 53. The Option Enum

- Rust doesn't have `null`, `nil`, or `undefined`
- Instead, we get a built-in enum called `Option`
- Has two variants - `Some` and `None`
- if you want to work with `Option` you have to use \
  pattern matching (the `if let` thing) or a match statement
- Forces you to handle the case in which you have a value\
  and the case in which you don't

```rust
enum Option {
    Some(u32),
    None,
}
```

### 56. Other Ways of Handling Options

- `item.unwrap()`
  - if `item` is a None, panics!
  - Use for quick debugging or examples
- `item.expect("There should be a value here")`
  - if `item` is a None, prints the provided debug message and panics!
  - Use When we **want** to crash if there is no value
- `item.unwrap_or(&placeholder)`
  - if `item` is a None, returns the provided default value
  - Use When it makes sense to provide a fallback value

[Rust Options](https://doc.rust-lang.org/std/option/)

## Section 6: Project Architecture: Mastering Modules in Rust

### 59. Modules Overview

#### Option 1: Create a mod in an existing file

- most appropriate when you have a really large file with a lot of stuff going on

#### Option 2: Create a module in a new single file in the same folder

- Most appropriate when you want to separate module to organise code,\
  but it doesn't need to span several files

#### Option 3: Spread code out among several separate files in a new folder

- most appropriate when you have a large module
- Has a couple of confusing parts

### 61. Refactoring with Multiple Modules

- `pub`: export
- `mod`: import
- `super`: parent

## Section 7: Handling the Unexpected: Errors and Results  

### 62. Project Overview

```sh
mkdir 07-errors-results
cd 07-errors-results
cargo new logs
cd logs
```

### 64. The Result Enum

```rust
enum Result {
    Ok(value),
    Err(error)
}
```

### 65. The Result Enum in Action (Generic)

```rust
fn device(a: f64, b: f64) -> Result<f64, Error>

enum Result <T, E> {
    Ok(T),
    Err(E)
}
```

### 68. Empty OK Variants

```rust
// empty tuple
Ok(())
```

### 73. The Stack and Heap

- Stack
  - Fast, but limited size (2-8MB)
- Heap
  - Slow, but can grow to store a lot of data
- Data: (called Data Segment or Rodata Secment or Static Segment)
  - Stores literal values that we write into our code

#### Super Common Pattern

- `Stack` stores metadata about a datastructure
- `Heap` stores the actual data
- Avoids running out of memory in the stack if the datastructure grows to hold a lot of data

```rust
let nums = vec![1, 2, 3, 4, 5]
```

#### Corner Case

- If a data structure owns another data structure, the child's metadata will be placed on the heap

```rust
let vec_of_numbers = vec![
  vec![1, 2, 3, 4, 5]
]
```

</details>
