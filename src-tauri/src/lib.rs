use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{PlayableId, SearchResult, SearchType},
    AuthCodeSpotify, Credentials, OAuth,
};
use std::{collections::HashSet, env};
use tiny_http::Server;

/// Formats a track name with title and artist.
pub fn format_track_name(title: &str, artist: &str) -> String {
    format!("{} - {}", title, artist)
}

/// Generates the Spotify authorization URL.
pub fn generate_spotify_auth_url() -> String {
    "https://accounts.spotify.com/authorize".to_string()
}

/// Called by both the Tauri GUI and the CLI.
/// Opens the browser for OAuth, catches the callback, creates the playlist.
pub async fn create_playlist_from_songs(songs: Vec<String>) -> Result<String, String> {
    dotenv::dotenv().ok();

    let client_id = env::var("SPOTIFY_CLIENT_ID")
        .map_err(|_| "Missing SPOTIFY_CLIENT_ID — check your .env file".to_string())?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")
        .map_err(|_| "Missing SPOTIFY_CLIENT_SECRET — check your .env file".to_string())?;
    let redirect_uri = env::var("SPOTIFY_REDIRECT_URI")
        .unwrap_or_else(|_| "http://127.0.0.1:8888/callback".to_string());

    let creds = Credentials::new(&client_id, &client_secret);
    let oauth = OAuth {
        redirect_uri,
        scopes: HashSet::from([
            "playlist-modify-public".to_string(),
            "playlist-modify-private".to_string(),
            "user-library-read".to_string(),
        ]),
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::new(creds, oauth);

    let auth_url = spotify
        .get_authorize_url(false)
        .map_err(|e| format!("Could not build auth URL: {e}"))?;

    // Open the browser automatically
    open::that(&auth_url)
        .map_err(|e| format!("Could not open browser. Visit manually:\n{auth_url}\n\nError: {e}"))?;

    // One-shot HTTP server catches the OAuth redirect
    let server = Server::http("127.0.0.1:8888")
        .map_err(|e| format!("Could not start callback server on port 8888: {e}"))?;

    let request = server
        .recv()
        .map_err(|e| format!("Error waiting for OAuth callback: {e}"))?;

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
        .respond(tiny_http::Response::from_string(
            "<html><body><h2>Authorized! You can close this tab.</h2></body></html>",
        ))
        .map_err(|e| e.to_string())?;

    if code.is_empty() {
        return Err("No authorization code in the callback URL. Did you approve the app?".to_string());
    }

    spotify
        .request_token(&code)
        .await
        .map_err(|e| format!("Token exchange failed: {e}"))?;

    let user = spotify
        .me()
        .await
        .map_err(|e| format!("Could not fetch Spotify profile: {e}"))?;

    let playlist = spotify
        .user_playlist_create(user.id, "My Playlist", Some(false), None, None)
        .await
        .map_err(|e| format!("Could not create playlist: {e}"))?;

    let mut added: Vec<String> = Vec::new();
    let mut not_found: Vec<String> = Vec::new();
    let mut track_uris: Vec<PlayableId> = Vec::new();

    for song in &songs {
        match spotify
            .search(song, SearchType::Track, None, None, Some(1), None)
            .await
        {
            Ok(SearchResult::Tracks(tracks)) => {
                if let Some(track) = tracks.items.first() {
                    if let Some(id) = track.id.as_ref() {
                        track_uris.push(id.clone().into());
                        added.push(track.name.clone());
                    } else {
                        not_found.push(song.clone());
                    }
                } else {
                    not_found.push(song.clone());
                }
            }
            Ok(_) => not_found.push(song.clone()),
            Err(e) => return Err(format!("Search error for '{song}': {e}")),
        }
    }

    if track_uris.is_empty() {
        return Err("No tracks found — check your song names and try again.".to_string());
    }

    spotify
        .playlist_add_items(playlist.id, track_uris, None)
        .await
        .map_err(|e| format!("Could not add tracks to playlist: {e}"))?;

    let mut msg = format!("✅ Created playlist with {} track(s):\n", added.len());
    for t in &added {
        msg.push_str(&format!("  • {t}\n"));
    }
    if !not_found.is_empty() {
        msg.push_str(&format!("\n⚠️  Not found:\n"));
        for t in &not_found {
            msg.push_str(&format!("  • {t}\n"));
        }
    }

    Ok(msg)
}