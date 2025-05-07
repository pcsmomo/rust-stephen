# Rust: The Complete Developer's Guide

Rust: The Complete Developer's Guide by Stephen Grider

## Folder structure

- 01-foundations
  - section 1
  - section 2

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

</details>
