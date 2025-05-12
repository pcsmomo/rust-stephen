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
