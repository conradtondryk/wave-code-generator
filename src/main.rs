use wave_code_generator::generate_wave_codes_page;

fn main() {
    // Example track IDs
    let track_ids = vec![
        "69Kzq3FMkDwiSFBQzRckFD".to_string(),
        "3wUMcPzXcmaeW8QxTdyXQO".to_string(),
        "6LUGvXEAK8WxIBYK43uoTb".to_string(),
        "0ofHAoxe9vBkTCp2UQIavz".to_string(),
    ];

    // Generate HTML page
    let html = generate_wave_codes_page(&track_ids, Some("Example Wave Codes"));

    // Save to output folder
    match std::fs::write("output/example.html", html) {
        Ok(_) => println!(
            "Generated output/example.html with {} tracks!",
            track_ids.len()
        ),
        Err(e) => eprintln!("Failed to write HTML file: {}", e),
    }
}
