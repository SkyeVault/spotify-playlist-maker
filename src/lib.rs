use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{PlayableId, SearchResult, SearchType},
    AuthCodeSpotify, Credentials, OAuth,
};
use std::{collections::HashSet, env};
use tiny_http::Server;

pub fn format_track_name(title: &str, artist: &str) -> String {
    format!("{} - {}", title, artist)
}

pub fn generate_spotify_auth_url() -> String {
    "https://accounts.spotify.com/authorize".to_string()
}

pub async fn create_playlist_from_songs(playlist_name: String, songs: Vec<String>) -> Result<String, String> {
    dotenv::dotenv().ok();

    let client_id = env::var("SPOTIFY_CLIENT_ID")
        .map_err(|_| "Missing SPOTIFY_CLIENT_ID in .env".to_string())?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")
        .map_err(|_| "Missing SPOTIFY_CLIENT_SECRET in .env".to_string())?;
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

    let url_clone = auth_url.clone();
    std::thread::spawn(move || {
        if let Err(e) = open::that(&url_clone) {
            eprintln!("Could not open browser: {e}");
        }
    });

    let (tx, rx) = std::sync::mpsc::channel::<Result<String, String>>();

    std::thread::spawn(move || {
        let server: tiny_http::Server = match Server::http("127.0.0.1:8888") {
            Ok(s) => s,
            Err(e) => {
                let _ = tx.send(Err(format!("Could not start callback server: {e}")));
                return;
            }
        };

        match server.recv() {
            Ok(request) => {
                let url = request.url().to_string();
                let code = url
                    .split("code=")
                    .nth(1)
                    .unwrap_or("")
                    .split('&')
                    .next()
                    .unwrap_or("")
                    .to_string();

                let _ = request.respond(tiny_http::Response::from_string(
                    "<html><body><h2>Authorized! You can close this tab.</h2></body></html>",
                ));

                if code.is_empty() {
                    let _ = tx.send(Err("No authorization code received.".to_string()));
                } else {
                    let _ = tx.send(Ok(code));
                }
            }
            Err(e) => {
                let _ = tx.send(Err(format!("Callback server error: {e}")));
            }
        }
    });

    let code = tokio::task::spawn_blocking(move || {
        rx.recv().map_err(|e| format!("Channel error: {e}"))?
    })
    .await
    .map_err(|e| format!("Thread error: {e}"))??;

    spotify
        .request_token(&code)
        .await
        .map_err(|e| format!("Token exchange failed: {e}"))?;

    let user = spotify
        .me()
        .await
        .map_err(|e| format!("Could not fetch Spotify profile: {e}"))?;

    let playlist = spotify
        .user_playlist_create(user.id, &playlist_name, Some(false), None, None)
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
        .map_err(|e| format!("Could not add tracks: {e}"))?;

    let mut msg = format!("✅ Created playlist with {} track(s):\n", added.len());
    for t in &added {
        msg.push_str(&format!("  • {t}\n"));
    }
    if !not_found.is_empty() {
        msg.push_str("\n⚠️  Not found:\n");
        for t in &not_found {
            msg.push_str(&format!("  • {t}\n"));
        }
    }

    Ok(msg)
}
