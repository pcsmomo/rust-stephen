
# Rust, WebAssembly and React App playground

## Rust Side

### Install rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install wasm-pack

wasm-pack is a tool that helps you build and publish Rust-generated WebAssembly. Install it using:

```sh
cargo install wasm-pack
```

### Create a new rust project

```sh
cargo new --lib rust-wasm
cd rust-wasm
```

## React Side

### Create a react project with Vite

```sh
pnpm create vite@latest rust-wasm-react -- --template react-ts
```
