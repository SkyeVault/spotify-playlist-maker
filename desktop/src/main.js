const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
import { invoke } from "@tauri-apps/api/core";

const btn = document.querySelector("#run");
const out = document.querySelector("#out");

btn.addEventListener("click", async () => {
  out.textContent = "Running...";
  try {
    const res = await invoke("make_playlist");
    out.textContent = res;
  } catch (e) {
    out.textContent = String(e);
  }
});

import { invoke } from "@tauri-apps/api/core";

document.querySelector("#run").addEventListener("click", async () => {
  const out = document.querySelector("#out");
  out.textContent = "Running...";
  try {
    const res = await invoke("make_playlist");
    out.textContent = res;
  } catch (e) {
    out.textContent = String(e);
  }
});
