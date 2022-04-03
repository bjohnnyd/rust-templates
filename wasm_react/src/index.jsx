import React, { useState } from "react";
import ReactDOM from "react-dom";

ReactDOM.render(<h1> Hello, World!</h1>, document.getElementById("root"));

const wasm = import("../build/{{crate_name}}");

wasm.then((m) => {
  const App = () => {
    const [name, setName] = useState("");
    const handleChange = (e) => {
      setName(e.target.value);
    };
    const handleClick = () => {
      m.welcome(name);
    };

    return (
      <>
        <div>
          <h1>Hi there</h1>
          <button onClick={m.big_computation}>Run Computation</button>
        </div>
        <div>
          <input type="text" onChange={handleChange} />
          <button onClick={handleClick}>Say hello!</button>
        </div>
      </>
    );
  };

  // This is necessary to drive the wasm promise to completion
  // as otherwise it wont be called
  ReactDOM.render(<App />, document.getElementById("root"));
});
