
import { invoke } from "@tauri-apps/api/core";

const songsEl = document.querySelector("#songs");
const outEl = document.querySelector("#out");
const saveBtn = document.querySelector("#save");
const runBtn = document.querySelector("#run");

function parseSongs(text) {
  return text
    .split("\n")
    .map((s) => s.trim())
    .filter(Boolean);
}

function setStatus(msg) {
  outEl.textContent = msg || "";
}

async function loadIntoEditor() {
  setStatus("Loading songs...");
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
    setStatus(String(e));
  }
});

runBtn.addEventListener("click", async () => {
  try {
    const songs = parseSongs(songsEl.value);
    if (songs.length === 0) {
      setStatus("Add at least one song (one per line).");
      return;
    }
    setStatus("Creating playlist...");
    const res = await invoke("make_playlist", { songs });
    setStatus(res);
  } catch (e) {
    setStatus(String(e));
  }
});

// boot
loadIntoEditor().catch((e) => setStatus(String(e)));
