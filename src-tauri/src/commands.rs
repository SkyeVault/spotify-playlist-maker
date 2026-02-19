use dotenvy::dotenv;
use rspotify::{
  clients::{BaseClient, OAuthClient},
  model::{SearchResult, SearchType},
  AuthCodeSpotify, Credentials, OAuth,
};
use std::{collections::HashSet, env};
use tiny_http::Server;

#[tauri::command]
pub async fn create_demo_playlist() -> Result<String, String> {
  dotenv().ok();

  let creds = Credentials::new(
    &env::var("SPOTIFY_CLIENT_ID").map_err(|e| e.to_string())?,
    &env::var("SPOTIFY_CLIENT_SECRET").map_err(|e| e.to_string())?,
  );

  let oauth = OAuth {
    redirect_uri: env::var("SPOTIFY_REDIRECT_URI").map_err(|e| e.to_string())?,
    scopes: HashSet::from([
      "playlist-modify-public".to_string(),
      "playlist-modify-private".to_string(),
      "user-library-read".to_string(),
    ]),
    ..Default::default()
  };

  let spotify = AuthCodeSpotify::new(creds, oauth);

  let auth_url = spotify.get_authorize_url(false).map_err(|e| e.to_string())?;

  // Open the auth URL (Linux/macOS/Windows) - dev convenience
  // If this ever fails in your environment, we’ll switch to a shell plugin open from the frontend.
  let _ = open::that(&auth_url);

  // Receive callback code on localhost:8888
  let server = Server::http("127.0.0.1:8888").map_err(|e| e.to_string())?;
  let request = server.recv().map_err(|e| e.to_string())?;
  let url = request.url().to_string();

  let code = url
    .split("code=")
    .nth(1)
    .unwrap_or("")
    .split('&')
    .next()
    .unwrap_or("")
    .to_string();

  request
    .respond(tiny_http::Response::from_string("You can close this tab now."))
    .map_err(|e| e.to_string())?;

  spotify.request_token(&code).await.map_err(|e| e.to_string())?;

  let user = spotify.me().await.map_err(|e| e.to_string())?;
  let playlist = spotify
    .user_playlist_create(user.id, "Tauri Demo Playlist", Some(false), None, None)
    .await
    .map_err(|e| e.to_string())?;

  let song_titles = [
    "Never Gonna Give You Up Rick Astley",
    "MMM Bop Hansen",
    "Feeling Good Nina Simone",
  ];

  let mut track_uris = Vec::new();
  for song in &song_titles {
    let search_result = spotify
      .search(song, SearchType::Track, None, None, Some(1), None)
      .await
      .map_err(|e| e.to_string())?;

    if let SearchResult::Tracks(tracks) = search_result {
      if let Some(track) = tracks.items.first() {
        if let Some(track_id) = track.id.as_ref() {
          track_uris.push(track_id.clone().into());
        }
      }
    }
  }

  if !track_uris.is_empty() {
    spotify
      .playlist_add_items(playlist.id.clone(), track_uris, None)
      .await
      .map_err(|e| e.to_string())?;
  }

  Ok(format!("Created playlist: {}", playlist.id))
}
