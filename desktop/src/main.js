const { invoke } = window.__TAURI__.core;

const songsEl = document.querySelector("#songs");
const outEl = document.querySelector("#out");
const saveBtn = document.querySelector("#save");
const runBtn = document.querySelector("#run");
const nameEl = document.querySelector("#playlist-name");

function parseSongs(text) {
  return text.split("\n").map((s) => s.trim()).filter(Boolean);
}

function setStatus(msg) {
  outEl.textContent = msg || "";
}

async function loadIntoEditor() {
  setStatus("Loading saved songs...");
  const songs = await invoke("load_songs");
  songsEl.value = (songs || []).join("\n");
  setStatus("");
}

saveBtn.addEventListener("click", async () => {
  try {
    setStatus("Saving...");
    const songs = parseSongs(songsEl.value);
    await invoke("save_songs", { songs });
    setStatus(`Saved ${songs.length} song(s).`);
  } catch (e) {
    setStatus("Error: " + String(e));
  }
});

runBtn.addEventListener("click", async () => {
  try {
    const songs = parseSongs(songsEl.value);
    if (songs.length === 0) {
      setStatus("Add at least one song (one per line).");
      return;
    }
    const playlistName = nameEl.value.trim() || "My Playlist";
    setStatus("Opening Spotify in your browser...\nApprove access, then come back here.");
    const res = await invoke("make_playlist", { playlistName: playlistName, songs });
    setStatus(res);
  } catch (e) {
    setStatus("Error: " + String(e));
  }
});

loadIntoEditor().catch((e) => setStatus("Error: " + String(e)));