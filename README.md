# Rust: The Complete Developer's Guide

Rust: The Complete Developer's Guide by Stephen Grider

## Folder structure

- 01-foundations/deck
  - section 1 and 2
- 03-rust-memory-system
  - bank
    - section 3 and 4
  - comparison-js-rust: comparing javascript and rust memory system
- 05-enums/media
  - section 5 and 6
- 07-errors-results/logs
- 08-iterator/iter
- 09-lifetimes/lifetimes
- 10-generics-traits
  - generics
    - first part of section 10
  - traits
    - second part of section 10

## Details

<details open>
  <summary>Click to Contract/Expend</summary>

## Section 1: Foundations of Rust: Setup and First Steps

### 2. Rust Installation

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo --version
# cargo 1.86.0 (adf9b6ad1 2025-02-28)

rustc --version
# rustc 1.86.0 (05f9846f8 2025-03-31)
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

### 74. Strings, String Refs, and String Slices

- `String`
  - Use anytime we want ownership of text
  - Use anytime we want text that can grow or shrink
- `&String`: String reference
  - Rarely used!
  - Rust will automatically turn `&String` into `&str` for you
- `&str`: String slice
  - Use anytime you don't want to take ownership of text
  - Use anytime you want to refer to a `portion` of a string owned by something else

  ```rust
  let color = String::from("red");
  let c = color.as_str();
  ```

  - Reason #1: `&str` lets you refer to text in the `data segment` without a `heap` allocation
    - case 1:

      - slightly better performance.

      ```rust
      let color = "red";
      ```

    - case 2: `"String::from("red").as_str()`

      ```rust
      let color = String::from("red");
      let color_ref = &color;
      ```

  - Reason #2: `&str` lets you `slice` (take a portion) of text that is already on the heap

    ```rust
    let color = String::from("blue");
    let portion = &color[1..4]; // "lue"
    ```

    - without `&str`: there are extra allocations involved. (not good in performance)

    ```rust
    let color = String::from("blue");
    let portion = String::from(
      color.chars().skip(1).collect::<String>();
    );
    let portion_ref = &portion;
    ```

### 75. When to Use Which String

Summary

| Name      | When to use                                                                                        | Use memory in... | Notes                                                                   |
|:---------:|:--------------------------------------------------------------------------------------------------:|:----------------:|:-----------------------------------------------------------------------:|
| `String`  | When you want to take ownership of text data.<br> When you have a string that might grow or shrink | Stack and Heap   |                                                                         |
| `&String` | Usually never                                                                                      | Stack            | Rust automatically turns `String` into a `&str` for you                 |
| `&str`    | When you want to read all or a portion of some text owned by something else                        | Stack            | Refers directly to heap-allocated or data-allocated text                |

### 77. Understanding the Issue

```py
text = "how are you"
word_list = text.split(" ")
# stores "how" , "are", "you"
```

```rust
let text = "how are you"
let split_text = text.split(" ");
// stores &str, &str, &str
```

#### Lifetime error

```rust
`text_that_was_read` does not live long enough
```

### 81. The Try Operator

- `?`: [Try Operator](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html)

```rust
fn main() -> Result<(), Error> {
  let text = fs::read_to_string("logs.txt")?;
}
```

### 82. When to Use Each Technique

1. Use a `match` or `if let` statement
   - When you're ready to meaningfully deal with an error
   - [example.rs](./07-errors-results/logs/lectures/82-1-match.rs)
2. Call `unwrap()` or `expect("why this paniced")` on the Result
   - Quick debugging, or if you wawnt to crash on an Err()
3. Use the try operator(`?`) to unwrap or propagate the Result
   - When you don't have any way to handle the error in the current function
   - [example.rs](./07-errors-results/logs/lectures/82-3-try-operator.rs)

## Section 8: Iterator Deep Dive: Efficient Data Processing

### 83. Project Overview

```sh
mkdir 08-iterator
cd 08-iterator
cargo new iter
cd iter
```

|       Name        | Description                                                            |
| :---------------: | ---------------------------------------------------------------------- |
| shorten_strings() | [90-iter_mut.rs](./08-iterator/iter/lectures/90-iter_mut.rs)           |
|  move_elements()  | [94-into_iter.rs](./08-iterator/iter/lectures/94-into_iter.rs)         |
|  print_element()  | [88-vector-slices.rs](./08-iterator/iter/lectures/88-vector-slices.rs) |
|  to_uppercase()   | [92-collect.rs](./08-iterator/iter/lectures/92-collect.rs)             |
|     explode()     | [95-inner-maps.rs](./08-iterator/iter/lectures/95-inner-maps.rs)       |
|  find_color_or()  | [97-find-map_or.rs](./08-iterator/iter/lectures/97-find-map_or.rs)     |

### 86. Iterator Consumers

iterator is lazy. Nothing happens until...

- A) You call `next()`
- B) You use a function that called `next()` automatically
  - `iterator consumers` such as `for_each()`, `collect()`,  etc

### 87. Iterator Adaptors

However, `map()` is not consumer but `iterator adaptor`\
and it doesn't call `next()` automatically

```rust
elements.iter().map(|el| format!("{} {}", el, el)); // error
```

### 88. Vector Slices

```rust
// expect full vector
fn print_elements(elements: &Vec<String>) {}

// expect slice (full vector or part of vector)
fn print_elements(elements: &[String]) {}


fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    print_elements(&colors[1..3]);
    // print_elements(&colors);
}
```

### 90. Iterators with Mutable Refs

- `iter`: **read-only reference**
- `iter_mut()`: **mutable reference**
- `into_iter()`: **ownership**, unless called on a mutable ref to a vector

### 93. How Collect Works

- collect decides the return type automatically following examples

```rust
// 1. collect() will follow the return type of this function
fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}

// 2. collect() will follow the type specified for uppercased
fn to_uppercase(elements: &[String]) -> Vec<String> {
    let uppercased: Vec<String> = elements.iter().map(|el| el.to_uppercase()).collect();
    uppercased
}

// 3. Turbofish, and Stephen's preferred way. it's obvious next to collect()
fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}
```

`Vec<String>` can be used like `Vec<_>` as `collect()` knows the the return type in the previous chain

```rust
fn to_uppercase(elements: &[String]) -> Vec<String> {
    let uppercased: Vec<_> = elements.iter().map(|el| el.to_uppercase()).collect();
    uppercased
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<_>>()
}
```

## Section 9: Advanced Lifetimes: Mastering Rust's Memory Model

### 102. Lifetime Annotations

```sh
mkdir 09-lifetimes
cd 09-lifetimes
cargo new lifetimes
cd lifetimes
```

```rust
struct Account {
  balance: i32
}

struct Bank<'a> {
  primary_account: &'a Account
}
```

```rust
fn longest<'a>(str_a: &'a str, str_b: &'a str) -> &'a str {
  if str_a.len() > str_b.len() {
    str_a
  } else {
    str_b
  }
}
```

### 105. What Lifetime Annotation Are All About

- when there are **more than two ref arguments**, Rust will assume the return would be one of the arguments
- Rust will not analyse the body of your function to figure out whether the return ref is pointing at the first or second arg

```rust
fn next_language(languages: &[String], current: &str) -> &str {}
```

- To clarify which ref the return ref is pointing at, we have to add lifetime annotations
  - `a` in `'a` is just a identifier, so it can be `'LifetimeAnnotation`, but in developer convention, it usually be `'a`

```rust
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {}
```

### 107. Lifetime Elision

You can omit annotations in two scenarios.

1. Function that takes one ref + any number of values + return a ref

   ```rust
   fn last_language(languages: &[String]) -> &str
   fn generate(set: &[i32], range: i32) -> &str
   fn leave(message: &Message, text: String) -> &str
   ```

2. Method that takes `&self` and any number of other refs + returns a ref.
**Rust assumes the returned ref will point at `&self`**

   ```rust
   struct Bank { 
     name: String
   }

   impl Bank {
     fn get_name(&self, default_name: &str) -> &str {
       &self.name
     }
   }
   ```

## Section 10: Generics and Traits: Writing Flexible, Reusable Code

### 109. Project Setup

```sh
mkdir 10-generics-traits
cd 10-generics-traits
cargo new generics
cd generics
```

```sh
cargo add num-traits
```

### 112. Trait Bounds

A trait is a set of methods

- it can contain **abstract methods** which don't have an implementation
- it can contain **default methods**, which have an implementation

```rust
trait Vehicle {
  fn start(&self);

  fn stop(&self) {
    println!("Stopped");
  }
}
```

A struct/enum/primitive can **implement** a trait

- The implementor has to provide an implementation for all of the **abstract methods**
- The implmentor can **optionally** override the default methods

```rust
struct Car {};

impl Vehicle for Car {
  fn start(&self) {
    println!("Start!!!");
  }
}
```

Type `T` must be something that implements the Vehicle trait

```rust
fn start_and_stop<T: Vehicle>(vehicle: T) {
  vehicle.start();

  vehicle.stop();
}

fn main() {
  let car = Car {};

  start_and_stop(car);
}
```

### 114. Super Solve Flexibility

```sh
cargo new traits
cd traits
```

</details>
