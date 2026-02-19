mod commands;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![commands::make_playlist])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

