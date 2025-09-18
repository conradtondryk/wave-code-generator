//! # Wave Code Generator
//!
//! A Rust library for generating printable HTML pages with Spotify wave codes
//! arranged in a clean 4-column grid layout.
//!
//! ## Features
//!
//! - Generate HTML pages with Spotify wave codes from track IDs
//! - Clean, printable 4-column grid layout
//! - Print-optimized CSS with no margins or borders
//! - Modular design with separate functions for individual songs and complete pages
//!
//! ## Usage
//!
//! ```rust
//! use wave_code_generator::generate_wave_codes_page;
//!
//! let track_ids = vec![
//!     "69Kzq3FMkDwiSFBQzRckFD".to_string(),
//!     "3wUMcPzXcmaeW8QxTdyXQO".to_string(),
//! ];
//!
//! let html = generate_wave_codes_page(&track_ids, Some("My Playlist"));
//! std::fs::write("wave_codes.html", html).expect("Failed to write HTML file");
//! ```

use std::fmt::Write;

/// Configuration for generating wave codes
#[derive(Debug, Clone)]
pub struct WaveCodeConfig {
    /// Page title
    pub title: String,
    /// Grid columns (default: 4)
    pub columns: u32,
    /// Background color (default: "white")
    pub background_color: String,
    /// Image size (default: 640)
    pub image_size: u32,
}

impl Default for WaveCodeConfig {
    fn default() -> Self {
        Self {
            title: "Spotify Codes Printable Page".to_string(),
            columns: 4,
            background_color: "white".to_string(),
            image_size: 640,
        }
    }
}

/// Generate a single song div with Spotify wave code
///
/// # Arguments
///
/// * `track_id` - Spotify track ID (e.g., "69Kzq3FMkDwiSFBQzRckFD")
/// * `alt_text` - Alt text for the image (optional)
/// * `image_size` - Size of the Spotify code image (default: 640)
///
/// # Returns
///
/// HTML string for a single song div
pub fn generate_song_div(
    track_id: &str,
    alt_text: Option<&str>,
    image_size: Option<u32>,
) -> String {
    let alt = alt_text.unwrap_or("Spotify Code");
    let size = image_size.unwrap_or(640);
    let spotify_code_url = format!(
        "https://scannables.scdn.co/uri/plain/png/000000/white/{}/spotify:track:{}",
        size, track_id
    );

    format!(
        r#"    <div class="song">
        <img src="{}" alt="{}">
    </div>"#,
        spotify_code_url, alt
    )
}

/// Generate CSS styles for the wave codes page
///
/// # Arguments
///
/// * `config` - Configuration for the page layout
///
/// # Returns
///
/// CSS string with all necessary styles
pub fn generate_css(config: &WaveCodeConfig) -> String {
    format!(
        r#"        body {{
            font-family: Arial, sans-serif;
            margin: 10px;
            padding: 0;
            background-color: {};
            display: grid;
            grid-template-columns: repeat({}, 1fr);
            column-gap: 1px;
            row-gap: 1px;
        }}
        .song {{
            margin: 0;
            padding: 0;
            box-shadow: none;
            border-radius: 0;
            text-align: center;
            page-break-inside: avoid;
        }}
        img {{
            max-width: 100%;
            height: auto;
            border: none;
            border-radius: 0;
            display: block;
        }}
        @media print {{
            body {{ padding: 0; margin: 0; background: white; }}
            .song {{ margin: 0; box-shadow: none; border: none; }}
            @page {{
                margin: 10px;
            }}
        }}"#,
        config.background_color, config.columns
    )
}

/// Generate a complete HTML page with Spotify wave codes
///
/// # Arguments
///
/// * `track_ids` - Vector of Spotify track IDs
/// * `title` - Optional page title (defaults to "Spotify Codes Printable Page")
///
/// # Returns
///
/// Complete HTML page as a string
///
/// # Example
///
/// ```rust
/// use wave_code_generator::generate_wave_codes_page;
///
/// let track_ids = vec![
///     "69Kzq3FMkDwiSFBQzRckFD".to_string(),
///     "3wUMcPzXcmaeW8QxTdyXQO".to_string(),
/// ];
///
/// let html = generate_wave_codes_page(&track_ids, Some("My Playlist"));
/// ```
pub fn generate_wave_codes_page(track_ids: &[String], title: Option<&str>) -> String {
    let config = WaveCodeConfig {
        title: title.unwrap_or("Spotify Codes Printable Page").to_string(),
        ..Default::default()
    };
    generate_wave_codes_page_with_config(track_ids, &config)
}

/// Generate a complete HTML page with Spotify wave codes using custom configuration
///
/// # Arguments
///
/// * `track_ids` - Vector of Spotify track IDs
/// * `config` - Configuration for the page layout and styling
///
/// # Returns
///
/// Complete HTML page as a string
pub fn generate_wave_codes_page_with_config(
    track_ids: &[String],
    config: &WaveCodeConfig,
) -> String {
    // Generate all song divs
    let mut songs_html = String::new();
    for track_id in track_ids {
        let song_div = generate_song_div(track_id, None, Some(config.image_size));
        writeln!(songs_html, "{}", song_div).unwrap();
    }

    // Generate CSS
    let css = generate_css(config);

    // Complete HTML template
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
{}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        config.title,
        css,
        songs_html.trim_end()
    )
}

/// Load track IDs from a text file (one per line)
///
/// # Arguments
///
/// * `file_path` - Path to the text file containing track IDs
///
/// # Returns
///
/// Result containing vector of track IDs or an error
pub fn load_track_ids_from_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    let track_ids: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    Ok(track_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_song_div() {
        let track_id = "69Kzq3FMkDwiSFBQzRckFD";
        let result = generate_song_div(track_id, None, None);

        assert!(result.contains("spotify:track:69Kzq3FMkDwiSFBQzRckFD"));
        assert!(result.contains(r#"alt="Spotify Code""#));
        assert!(result.contains("640"));
    }

    #[test]
    fn test_generate_wave_codes_page() {
        let track_ids = vec![
            "69Kzq3FMkDwiSFBQzRckFD".to_string(),
            "3wUMcPzXcmaeW8QxTdyXQO".to_string(),
        ];

        let html = generate_wave_codes_page(&track_ids, Some("Test Page"));

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test Page"));
        assert!(html.contains("grid-template-columns: repeat(4, 1fr)"));
        assert!(html.contains("spotify:track:69Kzq3FMkDwiSFBQzRckFD"));
        assert!(html.contains("spotify:track:3wUMcPzXcmaeW8QxTdyXQO"));
    }

    #[test]
    fn test_default_config() {
        let config = WaveCodeConfig::default();
        assert_eq!(config.title, "Spotify Codes Printable Page");
        assert_eq!(config.columns, 4);
        assert_eq!(config.background_color, "white");
        assert_eq!(config.image_size, 640);
    }
}
