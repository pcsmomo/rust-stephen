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
- `let` is not actually variable, but binding

</details>
