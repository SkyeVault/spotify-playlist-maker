# Rust Spotify Playlist Maker

Rust Program Documentation

This Rust program automates playlist creation on Spotify using the `rspotify` library. It authenticates users, searches for songs, and creates a playlist with predefined tracks. Edit the playlist name and song selection choices inside the code at line 66 and 70 of `spotify_playlist_maker/src/main.rs`.

---

## Features
- OAuth2 authentication with Spotify (no need to manually enter tokens)
- Automated playlist creation
- Song search and addition
- Uses `tiny_http` to handle the authentication callback automatically

![Crates.io](https://img.shields.io/crates/d/spotify_playlist_maker)
![Crates.io](https://img.shields.io/crates/l/spotify_playlist_maker)
![Crates.io](https://img.shields.io/crates/v/spotify_playlist_maker)
---

## Prerequisites
Before running the program, make sure you have the following installed:

1. **Rust and Cargo** (if not already installed)  
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Spotify Developer Account**  
   - Go to [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
   - Create a new app
   - Add the following **Redirect URIs**:
     ```
     http://127.0.0.1:8888/callback
     http://localhost:8888/callback
     ```
   - Copy the **Client ID** and **Client Secret**.

---

## Installation and Setup

### Install via Cargo
https://crates.io/crates/spotify_playlist_maker

### Create a `.env` file for your API credentials
```sh
nano .env
```
Paste the following into the file:
```ini
SPOTIFY_CLIENT_ID="your_client_id_here"
SPOTIFY_CLIENT_SECRET="your_client_secret_here"
SPOTIFY_REDIRECT_URI="http://127.0.0.1:8888/callback"
SPOTIFY_SCOPES="playlist-modify-public playlist-modify-private user-library-read"
```
Save the file (`CTRL + X → Y → Enter`).

---

## Running the Program
1. **Build and run the program**
   ```sh
   cargo run
   ```
2. **Follow the authorization link in the terminal**
3. **Approve the application on Spotify**
4. **Once approved, the program will automatically create the playlist and add songs**

---

## Testing & CI/CD

This project includes:
- Unit tests for core logic (`cargo test`)
- Integration tests for API calls
- GitHub Actions for automated testing on every push

[![Rust CI](https://github.com/SkyeVault/Main/actions/workflows/ci.yml/badge.svg)](https://github.com/SkyeVault/Main/actions/workflows/ci.yml)

## Built With
- Rust
- rspotify (Spotify API client)
- tiny_http (to handle OAuth2 callback)

---

## License
This project is licensed under the MIT License.

---

## Maintained by
[SkyeVault](https://github.com/SkyeVault)
