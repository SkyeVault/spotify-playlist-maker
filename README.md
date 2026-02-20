# Spotify Playlist Maker

Automate Spotify playlist creation using Rust.

This project includes:

* A CLI tool (published on crates.io)
* A full Desktop UI built with Tauri
* OAuth2 authentication with Spotify
* Automated playlist creation from a song list

---

# Overview

Spotify Playlist Maker allows you to:

* Authenticate securely with Spotify
* Search for songs
* Create playlists automatically
* Manage songs through a desktop interface

The project is structured as:

```
spotify-playlist-maker/
│
├── src/                # Core Spotify logic (library + CLI)
│
└── desktop/            # Tauri desktop application
```

---

# CLI Installation (Crates.io)

If you only want the command-line tool:

```
cargo install spotify_playlist_maker
```

Verify installation:

```
spotify_playlist_maker --help
```

---

# Spotify Developer Setup

1. Visit: [https://developer.spotify.com/dashboard](https://developer.spotify.com/dashboard)
2. Create a new application
3. Add the following Redirect URI:

```
http://127.0.0.1:8888/callback
```

4. Copy your:

   * Client ID
   * Client Secret

---

# CLI Configuration

Create a `.env` file in the directory where you will run the CLI:

```
SPOTIFY_CLIENT_ID=your_client_id
SPOTIFY_CLIENT_SECRET=your_client_secret
SPOTIFY_REDIRECT_URI=http://127.0.0.1:8888/callback
SPOTIFY_SCOPES=playlist-modify-public playlist-modify-private user-library-read
```

Do not wrap values in quotes.

---

# Running the CLI

From the directory containing `.env`:

```
spotify_playlist_maker
```

You will:

1. Receive an authorization URL
2. Open it in your browser
3. Approve access
4. The playlist will be created automatically

---

# Desktop Application (Tauri)

The desktop UI is not included in the crates.io binary.
It requires cloning the repository.

Clone the project:

```
git clone https://github.com/SkyeVault/spotify-playlist-maker.git
cd spotify-playlist-maker/desktop
```

Install Linux dependencies (Ubuntu/Debian):

```
sudo apt install \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

Run in development mode:

```
cargo tauri dev
```

The desktop app allows you to:

* Edit your song list
* Save songs locally
* Create playlists from the UI

---

# Building the Desktop App for Distribution

From the `desktop/` directory:

```
cargo tauri build
```

Production builds will be generated in:

```
desktop/src-tauri/target/release/bundle/
```

You can distribute the generated `.deb`, `.AppImage`, or other platform-specific packages.

---

# Running from Source (CLI)

If you cloned the repository and want to run the CLI version:

```
cd spotify-playlist-maker
cargo run
```

Make sure `.env` exists in the project root.

---

# Troubleshooting

## Port 8888 Already in Use

If you see:

Address already in use

Run:

```
sudo ss -ltnp | grep ':8888'
sudo kill <pid>
```

Then try again.

---

# Built With

* Rust
* rspotify
* tiny_http
* Tauri

---

![Crates.io](https://img.shields.io/crates/d/spotify_playlist_maker)
![Crates.io](https://img.shields.io/crates/l/spotify_playlist_maker)
![Crates.io](https://img.shields.io/crates/v/spotify_playlist_maker)
---


## Maintained by
[SkyeVault](https://github.com/SkyeVault)

