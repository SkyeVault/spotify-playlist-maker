use tauri::Manager;

use std::{fs, io::Write, path::PathBuf};

fn default_songs() -> Vec<String> {
  vec![
    "Frank Zappa - Transylvania Boogie".into(),
    "Dio - Rainbow in the Dark".into(),
    "Primus - DMV".into(),
    "Hum - If You Are To Bloom".into(),
    "Slint - Charlotte".into(),
    "System Of A Down - Mind".into(),
    "TOOL - H.".into(),
    "Helmet - Bad Mood".into(),
    "Slint - Nosferatu Man".into(),
    "Melvins - Night Goat".into(),
    "Black Sabbath - Electric Funeral".into(),
    "CAN - Vitamin C".into(),
    "Rush - Vital Signs".into(),
    "Nirvana - Moist Vagina".into(),
    "Deftones - Engine No. 9".into(),
    "The Mothers Of Invention - Toads Of The Short Forest".into(),
    "Local H - Back In The Day".into(),
    "Minutemen - Toadies".into(),
    "Ben Root - Baby Where Are You".into(),
    "Neutral Milk Hotel - Gardenhead / Leave Me Alone".into(),
    "Tame Impala - Expectation".into(),
    "Low Animal - Pending Horror".into(),
    "Primus - Mr. Knowitall".into(),
    "Grover Washington, Jr. - Knucklehead".into(),
    "Billy Cobham - Stratus".into(),
    "Mahavishnu Orchestra - Vital Transformation".into(),
    "Weather Report - Black Market".into(),
    "John Coltrane - A Love Supreme, Pt. III - Pursuance".into(),
    "Nirvana - Paper Cuts".into(),
    "Linkin Park - By Myself".into(),
    "Weezer - Only In Dreams".into(),
    "Melvins - Respite".into(),
    "Helmet - Speechless".into(),
    "Fugazi - Returning The Screw".into(),
    "Green Day - Emenius Sleepus".into(),
    "Stone Temple Pilots - Meatplow".into(),
    "Radiohead - My Iron Lung".into(),
    "The Jesus Lizard - Mouth Breather".into(),
    "Helmet - Your Head".into(),
    "Wipers - Return of the Rat".into(),
    "The Smashing Pumpkins - Here Is No Why".into(),
    "Interpol - Obstacle 1".into(),
    "Faith No More - Midlife Crisis".into(),
    "System Of A Down - U-Fig".into(),
    "Jimi Hendrix - Power To Love".into(),
    "CAN - One More Night".into(),
    "Foo Fighters - Exhausted".into(),
    "my bloody valentine - Sometimes".into(),
    "Tame Impala - It Is Not Meant To Be".into(),
    "Tame Impala - Jeremy's Storm".into(),
    "Tame Impala - Keep On Lying".into(),
    "Tame Impala - Nothing That Has Happened So Far Has Been Anything We Could Control".into(),
    "Radiohead - Pulk/Pull Revolving Doors".into(),
    "Radiohead - In Limbo".into(),
    "Breadwinner - Mac's Oranges".into(),
    "Unwound - Corpse Pose".into(),
    "Kyuss - Demon Cleaner".into(),
    "Preoccupations - Death".into(),
    "Black Sabbath - Symptom of the Universe".into(),
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
async fn make_playlist(app: tauri::AppHandle, playlist_name: String, songs: Vec<String>) -> Result<String, String> {
  save_songs(app, songs.clone())?;
  spotify_playlist_maker::create_playlist_from_songs(playlist_name, songs).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![load_songs, save_songs, make_playlist])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
