# Wave Code Generator

A blazingly fast Rust library and CLI tool for generating printable HTML pages with Spotify wave codes arranged in a clean 4-column grid layout.

## Features

- 🚀 **Fast**: Written in Rust for maximum performance
- 🖨️ **Print-optimized**: Clean CSS with no margins or borders for perfect printing
- 📱 **Responsive**: Works great on desktop and mobile
- 🔧 **CLI & Library**: Use as a command-line tool or integrate into your Rust projects
- 📄 **Simple**: Uses plaintext input files (one track ID per line)
- 📁 **Organized**: Input files go in `input/` folder, output HTML files go in `output/` folder

## Installation

### As a CLI tool
```bash
cargo install --path .
```

### As a library dependency
Add this to your `Cargo.toml`:
```toml
[dependencies]
wave-code-generator = { path = "." }
```

## Usage

### Option 1: From Spotify Playlist URL (Recommended)

```bash
# Step 1: Extract track IDs from Spotify playlist
cargo run --bin get-song-ids -- \
  --url "https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M" \
  --client-id "your_client_id" \
  --client-secret "your_client_secret" \
  --output "my_playlist.txt"

# Step 2: Generate HTML from extracted tracks
cargo run --bin wave-gen -- -i my_playlist.txt -t "My Awesome Playlist"
```

### Option 2: From Manual Track List

```bash
# Create input/my_tracks.txt with track IDs (one per line), then:
wave-gen -i my_tracks.txt -t "My Playlist"

# Generate with custom output filename
wave-gen -i my_tracks.txt -o custom_name.html -t "Custom Title"
```

### CLI Options

#### `get-song-ids` (Playlist Extractor)
- `-u, --url`: Spotify playlist URL - **REQUIRED**
- `--client-id`: Spotify Client ID - **REQUIRED** (see [setup guide](spotify_setup.md))
- `--client-secret`: Spotify Client Secret - **REQUIRED** (see [setup guide](spotify_setup.md))
- `-o, --output`: Output filename (saved in input/ folder)

#### `wave-gen` (HTML Generator)
- `-i, --input`: Input text file with track IDs (one per line) - **REQUIRED**
- `-o, --output`: Output HTML filename (saved in output/ folder)
- `-t, --title`: Page title (default: "Spotify Codes Printable Page")

### Library Usage

#### Basic Usage
```rust
use wave_code_generator::generate_wave_codes_page;

let track_ids = vec![
    "69Kzq3FMkDwiSFBQzRckFD".to_string(),
    "3wUMcPzXcmaeW8QxTdyXQO".to_string(),
];

let html = generate_wave_codes_page(&track_ids, Some("My Playlist"));
std::fs::write("wave_codes.html", html)?;
```

#### Advanced Configuration
```rust
use wave_code_generator::{generate_wave_codes_page_with_config, WaveCodeConfig};

let config = WaveCodeConfig {
    title: "Custom Layout".to_string(),
    columns: 6,
    background_color: "white".to_string(),
    image_size: 320,
};

let html = generate_wave_codes_page_with_config(&track_ids, &config);
```

#### Loading from Files
```rust
use wave_code_generator::load_track_ids_from_file;

// From text file
let track_ids = load_track_ids_from_file("input/tracks.txt")?;
let html = generate_wave_codes_page(&track_ids, Some("My Playlist"));
std::fs::write("output/my_codes.html", html)?;
```

### Project Structure

```
wave-code-generator/
├── input/           # Put your track ID files here
│   └── example.txt  # One track ID per line
├── output/          # Generated HTML files go here
│   └── example.html # Generated wave code pages
└── src/             # Rust source code
```

### Running Examples

```bash
# Run the built-in example
cargo run --bin wave-code-generator

# Run the CLI tool
cargo run --bin wave-gen -- -i example.txt -t "Test Playlist"
```

## API Reference

### Core Functions

#### `generate_wave_codes_page(track_ids: &[String], title: Option<&str>) -> String`
Generate a complete HTML page with default 4-column layout.

#### `generate_wave_codes_page_with_config(track_ids: &[String], config: &WaveCodeConfig) -> String`  
Generate HTML page with custom configuration.

#### `generate_song_div(track_id: &str, alt_text: Option<&str>, image_size: Option<u32>) -> String`
Generate HTML for a single wave code.

### Utility Functions

#### `load_track_ids_from_file(file_path: &str) -> Result<Vec<String>, std::io::Error>`
Load track IDs from text file (one per line).

### Configuration

#### `WaveCodeConfig`
```rust
pub struct WaveCodeConfig {
    pub title: String,           // Page title
    pub columns: u32,            // Grid columns (1-12)
    pub background_color: String, // Background color
    pub image_size: u32,         // Spotify code image size
}
```

## Output

The generated HTML creates a clean, printable page with:
- Clean 4-column grid layout
- No margins or padding for print optimization  
- Responsive images that scale properly
- Print-specific CSS rules for clean output
- Perfect for creating physical collections of Spotify wave codes

## Development

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test
```

### CLI Usage
```bash
cargo run --bin wave-gen -- --help
```

## License

MIT License - feel free to use this for your Spotify code collections!
