#[tauri::command]
pub async fn make_playlist() -> Result<String, String> {
  spotify_playlist_maker::run().await
}
