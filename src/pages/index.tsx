import { sums } from "../lib/wasm_bg.wasm";
import { useState } from "react";

const App = () => {
  const [value, setValue] = useState(0);
  return (
    <>
      <input
        onChange={(e) => {
          const v = Number(e.target.value);
          !isNaN(v) && setValue(sums(v));
        }}
      />
      <p>{value}</p>
    </>
  );
};

export default App;