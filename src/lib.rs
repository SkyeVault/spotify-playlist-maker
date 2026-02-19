/// Formats a track name with title and artist.
pub fn format_track_name(title: &str, artist: &str) -> String {
    format!("{} - {}", title, artist)
}

/// Generates the Spotify authorization URL.
pub fn generate_spotify_auth_url() -> String {
    "https://accounts.spotify.com/authorize".to_string()
}

pub async fn create_playlist_from_songs(songs: Vec<String>) -> Result<String, String> {
  Ok(format!("Received {} songs.", songs.len()))
}
