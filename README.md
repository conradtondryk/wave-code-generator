# Wave Code Generator

A blazingly fast Rust library and CLI tool for generating printable HTML pages with Spotify wave codes arranged in a clean, customizable grid layout.

## Features

- 🚀 **Fast**: Written in Rust for maximum performance
- 🎨 **Customizable**: Configurable grid layouts (1-12 columns), image sizes, and styling
- 🖨️ **Print-optimized**: Clean CSS with no margins or borders for perfect printing
- 📱 **Responsive**: Works great on desktop and mobile
- 🔧 **CLI & Library**: Use as a command-line tool or integrate into your Rust projects
- 📄 **Multiple input formats**: Support for direct input, text files, and JSON files

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

### Command Line Interface

```bash
# Generate from comma-separated track IDs
wave-gen -t "69Kzq3FMkDwiSFBQzRckFD,3wUMcPzXcmaeW8QxTdyXQO" -o my_codes.html

# Generate from text file (one track ID per line)
wave-gen -f track_ids.txt --title "My Playlist" --columns 3

# Generate from JSON file
wave-gen -j tracks.json --size 800 -o large_codes.html

# Use example data if no input provided
wave-gen
```

#### CLI Options
- `-t, --tracks`: Comma-separated track IDs
- `-f, --file`: Text file with track IDs (one per line)  
- `-j, --json`: JSON file with array of track IDs
- `-o, --output`: Output HTML file (default: wave_codes.html)
- `--title`: Page title (default: "Spotify Codes Printable Page")
- `-c, --columns`: Grid columns (default: 4)
- `-s, --size`: Image size (default: 640)

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
use wave_code_generator::{load_track_ids_from_file, load_track_ids_from_json};

// From text file
let track_ids = load_track_ids_from_file("tracks.txt")?;

// From JSON file  
let track_ids = load_track_ids_from_json("tracks.json")?;
```

### Running Examples

```bash
# Run the basic example
cargo run --example basic_usage

# Run advanced configuration examples
cargo run --example advanced_config

# Run file input examples
cargo run --example from_file

# Run the main binary with example data
cargo run
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

#### `load_track_ids_from_json(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>`
Load track IDs from JSON array file.

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
- Customizable grid layout (1-12 columns)
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

### Running Examples
```bash
cargo run --example basic_usage
cargo run --example advanced_config  
cargo run --example from_file
```

### CLI Usage
```bash
cargo run --bin wave-gen -- --help
```

## License

MIT License - feel free to use this for your Spotify code collections!
