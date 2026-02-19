import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const [status, setStatus] = useState("");

  async function createPlaylist() {
    setStatus("Starting OAuth... (your browser will open)");
    try {
      const msg = await invoke("create_demo_playlist");
      setStatus(msg);
    } catch (e) {
      setStatus(String(e));
    }
  }

  return (
    <div style={{ padding: 24, fontFamily: "system-ui" }}>
      <h1>Spotify Playlist Maker (Tauri)</h1>
      <button onClick={createPlaylist}>Create Demo Playlist</button>
      {status && <p style={{ marginTop: 12 }}>{status}</p>}
      <p style={{ opacity: 0.7, marginTop: 16 }}>
        Redirect URI expected: <code>http://127.0.0.1:8888/callback</code>
      </p>
    </div>
  );
}
