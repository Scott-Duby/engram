import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

// Placeholder screen: proves the React UI, daisyUI styling, and the Rust bridge all work.
function App() {
  const [name, setName] = useState("");
  const [reply, setReply] = useState("");

  async function greet() {
    setReply(await invoke<string>("greet", { name }));
  }

  return (
    <main className="min-h-screen bg-base-200 flex items-center justify-center p-6">
      <div className="card bg-base-100 shadow-sm w-full max-w-md">
        <div className="card-body gap-4">
          <h1 className="card-title text-2xl">Agent Brain</h1>
          <p className="text-base-content/70">Companion app scaffold. Voice, face, chat, and the display come next.</p>
          <form
            className="join w-full"
            onSubmit={(e) => {
              e.preventDefault();
              greet();
            }}
          >
            <input
              id="greet-input"
              className="input join-item w-full"
              value={name}
              onChange={(e) => setName(e.currentTarget.value)}
              placeholder="Type a name to test the Rust bridge"
            />
            <button type="submit" className="btn btn-primary join-item">Send</button>
          </form>
          {reply && <div role="status" className="alert alert-success">{reply}</div>}
        </div>
      </div>
    </main>
  );
}

export default App;
