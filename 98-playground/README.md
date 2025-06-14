
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

### Work on the project

### Build the rust project

```sh
wasm-pack build --target web

# [INFO]: Optimizing wasm binaries with `wasm-opt`...
# [INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
# [INFO]: ✨   Done in 41.58s
# [INFO]: 📦   Your wasm pkg is ready to publish at ~/rust-stephen/rust-stephen-git/98-playground/rust-wasm/pkg.
```

## React Side

### Create a react project with Vite

```sh
pnpm create vite@latest rust-wasm-react -- --template react-ts
```

### Install wasm-pack Output

Copy the contents of the `pkg` directory from `your Rust project` to your `React project`'s `src` directory. Then, install the package:

```sh
pnpm install ./src/pkg
```

### Use it in the app

```jsx
import { useEffect, useState } from "react";

import init, { greet } from "./pkg/rust_wasm";

function App() {
  const [greeting, setGreeting] = useState("");

  useEffect(() => {
    init().then(() => {
      setGreeting(greet("Noah!"));
    });
  }, []);

  return <div>{greeting}</div>;
}

export default App;
```
