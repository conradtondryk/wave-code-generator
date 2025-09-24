# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Wave Code Generator is a dual-component application that generates printable HTML pages with Spotify wave codes:
- **Rust Backend**: Core CLI tools for extracting Spotify track IDs and generating HTML pages
- **Next.js Frontend**: Web UI for interactive wave code generation with customization options

## Commands

### Rust Backend

```bash
# Build the project
cargo build --release

# Run tests
cargo test

# Lint with clippy
cargo clippy --all-targets

# Extract track IDs from Spotify playlist
cargo run --bin get-song-ids -- \
  --url "https://open.spotify.com/playlist/..." \
  --client-id "CLIENT_ID" \
  --client-secret "CLIENT_SECRET" \
  --output "playlist.txt"

# Generate HTML from track IDs
cargo run --bin wave-gen -- -i playlist.txt -t "Playlist Title"
```

### Next.js Frontend

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Development server
npm run dev

# Build for production
npm run build

# Start production server
npm start

# Lint TypeScript/React code
npm run lint
```

## Architecture

### Rust Components

The Rust backend consists of:

1. **Library (`src/lib.rs`)**: Core functionality for generating HTML pages with wave codes
   - `generate_wave_codes_page()`: Creates HTML with default 4-column layout
   - `generate_wave_codes_page_with_config()`: Custom configuration support
   - `WaveCodeConfig`: Struct for customizing columns, colors, image sizes

2. **CLI Binaries**:
   - `wave-gen` (`src/bin/main.rs`): Generates HTML from track ID text files
   - `get-song-ids` (`src/bin/get_song_ids.rs`): Extracts track IDs from Spotify playlists using the Spotify Web API

3. **File Structure**:
   - `input/`: Store track ID text files (one ID per line)
   - `output/`: Generated HTML files are saved here

### Frontend Components

The Next.js frontend (`frontend/`) provides:

1. **API Routes** (`src/app/api/`):
   - `extract-tracks/route.ts`: Calls Rust `get-song-ids` binary via child process
   - `generate-html/route.ts`: TypeScript implementation of wave code generation

2. **Main UI** (`src/app/page.tsx`): Interactive form with:
   - Spotify playlist URL input
   - Track ID extraction and editing
   - Customization controls (columns, size, color, title)
   - HTML generation and download

3. **Configuration**:
   - Uses environment variables for Spotify API credentials
   - Requires `.env.local` with `SPOTIFY_CLIENT_ID` and `SPOTIFY_CLIENT_SECRET`

## Key Implementation Details

### Wave Code Generation
- Uses Spotify's Scannables API: `https://scannables.scdn.co/uri/plain/png/{bg}/{fg}/{size}/spotify:track:{id}`
- Generates responsive CSS grid layout optimized for printing
- Default 4-column grid, customizable from 1-8 columns
- Image sizes range from 320px to 1280px

### Spotify Integration
- Requires Spotify App credentials from developer.spotify.com
- Uses client credentials OAuth flow for authentication
- Supports pagination for playlists with >100 tracks
- Extracts track IDs and saves to text files for processing

### Frontend-Backend Communication
- Frontend calls Rust binaries using Node.js child processes
- File-based communication through `input/` and `output/` directories
- TypeScript reimplementation allows direct HTML generation in frontend

## Development Workflow

1. **For Rust changes**: Run `cargo build` after modifications, test with `cargo test`
2. **For frontend changes**: Use `npm run dev` for hot-reload development
3. **Full stack testing**: Ensure both Rust backend is built and frontend server is running
4. **Spotify API setup**: Configure credentials in both environments if testing playlist extraction