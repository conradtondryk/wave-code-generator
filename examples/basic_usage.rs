//! Basic usage example for the wave code generator
//!
//! This example shows how to generate a simple HTML page with wave codes.

use wave_code_generator::generate_wave_codes_page;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example track IDs
    let track_ids = vec![
        "69Kzq3FMkDwiSFBQzRckFD".to_string(),
        "3wUMcPzXcmaeW8QxTdyXQO".to_string(),
        "6LUGvXEAK8WxIBYK43uoTb".to_string(),
        "0ofHAoxe9vBkTCp2UQIavz".to_string(),
    ];

    // Generate HTML page
    let html = generate_wave_codes_page(&track_ids, Some("My Favorite Songs"));

    // Save to file
    std::fs::write("basic_example.html", html)?;

    println!("Generated basic_example.html with {} tracks!", track_ids.len());
    println!("Open the HTML file in your browser to view the wave codes.");

    Ok(())
}
