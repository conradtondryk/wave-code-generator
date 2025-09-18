# Spotify API Setup

To use the `get-song-ids` script, you need Spotify API credentials:

## 1. Get Spotify API Credentials

1. Go to [Spotify Developer Dashboard](https://developer.spotify.com/dashboard/)
2. Log in with your Spotify account
3. Click "Create App"
4. Fill in:
   - **App name**: Wave Code Generator
   - **App description**: Extract track IDs from playlists
   - **Redirect URI**: http://localhost (not used but required)
5. Check the boxes and click "Save"
6. Copy your **Client ID** and **Client Secret**

## 2. Usage

```bash
# Extract track IDs from a Spotify playlist
cargo run --bin get-song-ids -- \
  --url "https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M" \
  --client-id "your_client_id_here" \
  --client-secret "your_client_secret_here" \
  --output "my_playlist.txt"

# Generate HTML from the extracted tracks
cargo run --bin wave-gen -- -i my_playlist.txt -t "My Awesome Playlist"
```

## 3. Complete Workflow

1. **Extract tracks**: `get-song-ids` → saves to `input/playlist_tracks.txt`
2. **Generate HTML**: `wave-gen` → saves to `output/playlist_tracks.html`

That's it! 🎵
