//! File input example
//!
//! This example shows how to load track IDs from text and JSON files.

use wave_code_generator::{generate_wave_codes_page, load_track_ids_from_file, load_track_ids_from_json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create example text file
    let sample_tracks_text = vec![
        "69Kzq3FMkDwiSFBQzRckFD",
        "3wUMcPzXcmaeW8QxTdyXQO", 
        "6LUGvXEAK8WxIBYK43uoTb",
        "0ofHAoxe9vBkTCp2UQIavz",
    ];
    
    let text_content = sample_tracks_text.join("\n");
    std::fs::write("sample_tracks.txt", text_content)?;

    // Create example JSON file
    let json_content = serde_json::to_string_pretty(&sample_tracks_text)?;
    std::fs::write("sample_tracks.json", json_content)?;

    // Example 1: Load from text file
    println!("Loading track IDs from text file...");
    let track_ids_from_text = load_track_ids_from_file("sample_tracks.txt")?;
    let html_from_text = generate_wave_codes_page(&track_ids_from_text, Some("Tracks from Text File"));
    std::fs::write("from_text_file.html", html_from_text)?;

    // Example 2: Load from JSON file  
    println!("Loading track IDs from JSON file...");
    let track_ids_from_json = load_track_ids_from_json("sample_tracks.json")?;
    let html_from_json = generate_wave_codes_page(&track_ids_from_json, Some("Tracks from JSON File"));
    std::fs::write("from_json_file.html", html_from_json)?;

    println!("Generated HTML files from both text and JSON inputs:");
    println!("- from_text_file.html ({} tracks from sample_tracks.txt)", track_ids_from_text.len());
    println!("- from_json_file.html ({} tracks from sample_tracks.json)", track_ids_from_json.len());

    // Clean up sample files
    std::fs::remove_file("sample_tracks.txt")?;
    std::fs::remove_file("sample_tracks.json")?;

    Ok(())
}
