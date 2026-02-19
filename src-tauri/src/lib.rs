cat > ~/GitHub/spotify-playlist-maker/desktop/src-tauri/src/lib.rs <<'EOF'
use std::{fs, io::Write, path::PathBuf};
use tauri::Manager;

fn default_songs() -> Vec<String> {
  vec![
    "Never Gonna Give You Up Rick Astley".into(),
    "MMM Bop Hanson".into(),
    "Feeling Good Nina Simone".into(),
  ]
}

fn songs_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
  let mut dir = app
    .path()
    .app_data_dir()
    .map_err(|e| format!("app_data_dir error: {e}"))?;

  fs::create_dir_all(&dir).map_err(|e| format!("create app data dir: {e}"))?;
  dir.push("songs.txt");
  Ok(dir)
}

#[tauri::command]
fn load_songs(app: tauri::AppHandle) -> Result<Vec<String>, String> {
  let path = songs_path(&app)?;

  if !path.exists() {
    let defaults = default_songs();
    save_songs(app, defaults.clone())?;
    return Ok(defaults);
  }

  let raw = fs::read_to_string(&path).map_err(|e| format!("read songs file: {e}"))?;
  Ok(raw
    .lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty())
    .map(|l| l.to_string())
    .collect())
}

#[tauri::command]
fn save_songs(app: tauri::AppHandle, songs: Vec<String>) -> Result<(), String> {
  let path = songs_path(&app)?;

  let mut f = fs::File::create(&path).map_err(|e| format!("create songs file: {e}"))?;
  for s in songs {
    let s = s.trim();
    if s.is_empty() {
      continue;
    }
    writeln!(f, "{s}").map_err(|e| format!("write songs file: {e}"))?;
  }
  Ok(())
}

#[tauri::command]
async fn make_playlist(app: tauri::AppHandle, songs: Vec<String>) -> Result<String, String> {
  save_songs(app, songs.clone())?;
  spotify_playlist_maker::create_playlist_from_songs(songs).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![load_songs, save_songs, make_playlist])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
EOF
