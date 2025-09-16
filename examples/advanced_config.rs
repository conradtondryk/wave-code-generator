//! Advanced configuration example
//!
//! This example shows how to use custom configuration for different layouts and styling.

use wave_code_generator::{generate_wave_codes_page_with_config, WaveCodeConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example track IDs
    let track_ids = vec![
        "69Kzq3FMkDwiSFBQzRckFD".to_string(),
        "3wUMcPzXcmaeW8QxTdyXQO".to_string(),
        "6LUGvXEAK8WxIBYK43uoTb".to_string(),
        "0ofHAoxe9vBkTCp2UQIavz".to_string(),
        "4mn2kNTqiGLwaUR8JdhJ1l".to_string(),
        "5e9TFTbltYBg2xThimr0rU".to_string(),
    ];

    // Example 1: 3-column layout with larger images
    let config_3col = WaveCodeConfig {
        title: "3-Column Layout".to_string(),
        columns: 3,
        background_color: "white".to_string(),
        image_size: 800,
    };

    let html_3col = generate_wave_codes_page_with_config(&track_ids, &config_3col);
    std::fs::write("3_column_example.html", html_3col)?;

    // Example 2: 6-column layout with smaller images
    let config_6col = WaveCodeConfig {
        title: "6-Column Compact Layout".to_string(),
        columns: 6,
        background_color: "white".to_string(),
        image_size: 320,
    };

    let html_6col = generate_wave_codes_page_with_config(&track_ids, &config_6col);
    std::fs::write("6_column_example.html", html_6col)?;

    // Example 3: Single column layout (good for mobile)
    let config_1col = WaveCodeConfig {
        title: "Mobile-Friendly Single Column".to_string(),
        columns: 1,
        background_color: "white".to_string(),
        image_size: 640,
    };

    let html_1col = generate_wave_codes_page_with_config(&track_ids, &config_1col);
    std::fs::write("single_column_example.html", html_1col)?;

    println!("Generated three different layout examples:");
    println!("- 3_column_example.html (3 columns, large images)");
    println!("- 6_column_example.html (6 columns, small images)");
    println!("- single_column_example.html (1 column, mobile-friendly)");

    Ok(())
}
