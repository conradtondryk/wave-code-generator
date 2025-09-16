//! Wave Code Generator CLI
//!
//! Command-line interface for generating printable HTML pages with Spotify wave codes.

use clap::{Arg, ArgMatches, Command};
use std::process;
use wave_code_generator::{
    generate_wave_codes_page, generate_wave_codes_page_with_config, load_track_ids_from_file,
    load_track_ids_from_json, WaveCodeConfig,
};

fn main() {
    let matches = Command::new("wave-gen")
        .version("0.1.0")
        .author("Rust Developer")
        .about("Generate printable HTML pages with Spotify wave codes")
        .arg(
            Arg::new("tracks")
                .short('t')
                .long("tracks")
                .value_name("TRACK_IDS")
                .help("Comma-separated list of Spotify track IDs")
                .conflicts_with_all(&["file", "json"]),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Text file with track IDs (one per line)")
                .conflicts_with_all(&["tracks", "json"]),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .value_name("JSON_FILE")
                .help("JSON file with array of track IDs")
                .conflicts_with_all(&["tracks", "file"]),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Output HTML file path")
                .default_value("wave_codes.html"),
        )
        .arg(
            Arg::new("title")
                .long("title")
                .value_name("TITLE")
                .help("Page title")
                .default_value("Spotify Codes Printable Page"),
        )
        .arg(
            Arg::new("columns")
                .short('c')
                .long("columns")
                .value_name("COLUMNS")
                .help("Number of columns in the grid")
                .default_value("4"),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .value_name("SIZE")
                .help("Image size for Spotify codes")
                .default_value("640"),
        )
        .get_matches();

    if let Err(e) = run(&matches) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // Load track IDs from various sources
    let track_ids = if let Some(tracks_str) = matches.get_one::<String>("tracks") {
        // Parse comma-separated track IDs
        tracks_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        // Load from text file
        load_track_ids_from_file(file_path)?
    } else if let Some(json_path) = matches.get_one::<String>("json") {
        // Load from JSON file
        load_track_ids_from_json(json_path)?
    } else {
        // Use example track IDs if no input provided
        println!("No input provided, using example track IDs...");
        vec![
            "69Kzq3FMkDwiSFBQzRckFD".to_string(),
            "3wUMcPzXcmaeW8QxTdyXQO".to_string(),
            "6LUGvXEAK8WxIBYK43uoTb".to_string(),
            "0ofHAoxe9vBkTCp2UQIavz".to_string(),
            "4mn2kNTqiGLwaUR8JdhJ1l".to_string(),
            "5e9TFTbltYBg2xThimr0rU".to_string(),
            "7w5AOd6HrDIHewHfpABEss".to_string(),
            "0oXJQ8CyDcQJXASZiCSNGa".to_string(),
            "0pUVeEgZuNyFzIMKp67RbS".to_string(),
            "1FvDJ9KGxcqwv1utyPL3JZ".to_string(),
            "1cWilR7SC3qyfl6emCvYf0".to_string(),
            "1YrnDTqvcnUKxAIeXyaEmU".to_string(),
            "1cHCG42MxckrXNFqyF8Uhr".to_string(),
            "7n3WO6ESKS1uCI9fgkGs66".to_string(),
            "2kkvB3RNRzwjFdGhaUA0tz".to_string(),
            "5QTxFnGygVM4jFQiBovmRo".to_string(),
        ]
    };

    if track_ids.is_empty() {
        return Err("No track IDs provided or found".into());
    }

    // Parse configuration
    let title = matches.get_one::<String>("title").unwrap();
    let columns: u32 = matches
        .get_one::<String>("columns")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid columns number")?;
    let image_size: u32 = matches
        .get_one::<String>("size")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid image size")?;

    // Create configuration
    let config = WaveCodeConfig {
        title: title.clone(),
        columns,
        background_color: "white".to_string(),
        image_size,
    };

    // Generate HTML
    let html = if columns != 4 || image_size != 640 {
        generate_wave_codes_page_with_config(&track_ids, &config)
    } else {
        generate_wave_codes_page(&track_ids, Some(title))
    };

    // Write to output file
    let output_path = matches.get_one::<String>("output").unwrap();
    std::fs::write(output_path, html)?;

    println!(
        "Generated {} with {} tracks in a {}-column grid",
        output_path,
        track_ids.len(),
        columns
    );

    Ok(())
}
