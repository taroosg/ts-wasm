import { sums, f, decimal_to_binary } from "../lib/wasm_bg.wasm";
import { useState } from "react";

const App = () => {
  const [value, setValue] = useState(0);
  return (
    <>
      <input
        type="number"
        onChange={(e) => {
          const v = Number(e.target.value);
          !isNaN(v) && setValue(sums(v));
          console.log(decimal_to_binary(v))
        }}
      />
      <p>{value}</p>
    </>
  );
};

export default App;