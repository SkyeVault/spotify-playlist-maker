use dotenv::dotenv;
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{PlayableId, SearchResult, SearchType},
    AuthCodeSpotify, Credentials, OAuth,
};
use spotify_playlist_maker::{format_track_name, generate_spotify_auth_url};
use std::{collections::HashSet, env, error::Error};
use tiny_http::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let formatted_track = format_track_name("Song Title", "Artist Name");
    println!("Formatted Track: {}", formatted_track);

    let auth_url = generate_spotify_auth_url();
    println!("Spotify Auth URL: {}", auth_url);

    let creds = Credentials::new(
        &env::var("SPOTIFY_CLIENT_ID")?,
        &env::var("SPOTIFY_CLIENT_SECRET")?,
    );

    let oauth = OAuth {
        redirect_uri: env::var("SPOTIFY_REDIRECT_URI")?.to_string(),
        scopes: HashSet::from([
            "playlist-modify-public".to_string(),
            "playlist-modify-private".to_string(),
            "user-library-read".to_string(),
        ]),
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::new(creds, oauth);

    let auth_url = spotify.get_authorize_url(false)?;
    println!("Authorize here: {}", auth_url);

    let server = Server::http("127.0.0.1:8888").unwrap();
    let request = server.recv().unwrap();
    let url = request.url().to_string();
    let code = url
        .split("code=")
        .nth(1)
        .unwrap_or("")
        .split('&')
        .next()
        .unwrap();

    println!("Received authorization code: {}", code);
    request
        .respond(tiny_http::Response::from_string(
            "You can close this tab now.",
        ))
        .unwrap();

    spotify.request_token(code).await?;
    println!("Authenticated successfully!");

    let user = spotify.me().await?;
    println!("Spotify User ID: {}", user.id);

    let playlist = spotify
        .user_playlist_create(user.id, "Rick Roll Playlist", Some(false), None, None)
        .await?;

    let song_titles = [
        "Never Gonna Give You Up Rick Astley",
        "MMM Bop Hansen",
        "Feeling Good Nina Simone",
    ];

    let mut track_uris: Vec<PlayableId> = Vec::new();

    for song in &song_titles {
        let search_result = spotify
            .search(song, SearchType::Track, None, None, Some(1), None)
            .await?;

        println!("Full SearchResult: {:#?}", search_result);

        if let SearchResult::Tracks(tracks) = search_result {
            if let Some(track) = tracks.items.first() {
                println!("Adding {} to playlist...", track.name);
                let track_id = track.id.as_ref().unwrap();
                track_uris.push(track_id.clone().into());
            }
        } else {
            println!("Could not find {}", song);
        }
    }

    if !track_uris.is_empty() {
        spotify
            .playlist_add_items(playlist.id, track_uris, None)
            .await?;
        println!("Added all songs successfully!");
    } else {
        println!("No songs were added.");
    }

    Ok(())
}
